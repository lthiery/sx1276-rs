#include "helium.h"
#include "board.h"

#include "radio/radio.h"
#include "radio/sx1276/sx1276.h"

#include <string.h> 
#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>

// [[radios]]
// id = 0
// type = 'SX1257'
// freq = 916_600_000
// rssi_offset = -169.0
// tx_enable = true

// [[radios]]
// id = 1
// type = 'SX1257'
// freq = 920_600_000
// rssi_offset = -169.0
// tx_enable = false

#define RADIO_1                                     916600000 // Hz //911_000_000 - 400_000
#define RADIO_2                                     920600000 // Hz //911_000_000 - 400_000
#define FREQ_SPACING                                   200000
const uint32_t frequency_table[8] = {
  RADIO_1 - FREQ_SPACING*2,
  RADIO_1 - FREQ_SPACING,
  RADIO_1,
  RADIO_2 - FREQ_SPACING*2,
  RADIO_2 - FREQ_SPACING,
  RADIO_2,
  RADIO_2 + FREQ_SPACING,
  RADIO_2 + FREQ_SPACING*2
};

#define TX_OUTPUT_POWER                             22        // dBm

#define LORA_BANDWIDTH                              0         // [0: 125 kHz,
                                                              //  1: 250 kHz,
                                                              //  2: 500 kHz,
                                                              //  3: Reserved]
#define LORA_SPREADING_FACTOR                       10         // [SF7..SF12]
#define LORA_CODINGRATE                             1         // [1: 4/5,
                                                              //  2: 4/6,
                                                              //  3: 4/7,
                                                              //  4: 4/8]
#define LORA_PREAMBLE_LENGTH                        8         // Same for Tx and Rx
#define LORA_SYMBOL_TIMEOUT                         5         // Symbols
#define LORA_FIX_LENGTH_PAYLOAD_ON                  false
#define LORA_IQ_INVERSION_ON                        false


typedef enum {
  InternalEvent_None,
  InternalEvent_TxDone,
  InternalEvent_RxDone,
  InternalEvent_TxTimeout,
  InternalEvent_RxTimeout,
  InternalEvent_RxError,
} InternalEvent_t;

#define NUM_IRQ_HANDLES   6

typedef struct {
  struct RfConfig config;
  void (*dio_irq_handles[NUM_IRQ_HANDLES])(void);
  RadioEvents_t radio_events;
  InternalEvent_t cur_event;
  uint8_t * buffer;
  size_t buffer_len;
  uint32_t rx_len;
} LongFi_t;

#define RX_TIMEOUT_VALUE                            1000
#define BUFFER_SIZE                                 64 // Define the payload size here

const uint8_t PingMsg[] = "PING";
const uint8_t PongMsg[] = "PONG";

uint16_t BufferSize = BUFFER_SIZE;
uint8_t Buffer[BUFFER_SIZE];

int16_t RssiValue = 0;
int8_t SnrValue = 0;

/*!
 * Given the constraints of the SX1276 API, we have to use static struct
 * It sets up the constraint of only one instance per system for now
 */
static LongFi_t LongFi;

/*
 *  Functions implemented here that SX1276 will call
 *  
 *  As far as I can tell, these are only called from SX1276 when:
 *      LongFi.dio_irq_handles are dispatched
 *      Timeout callbacks are dispatched
 *  With the event refactoring here, these callbacks will only be called from `helium_rf_handle_event`
 */
void OnTxDone( void );
void OnRxDone( uint8_t *payload, uint16_t size, int16_t rssi, int8_t snr );
void OnTxTimeout( void );
void OnRxTimeout( void );
void OnRxError( void );

/*
 * Private helper for handling internal events
 */
ClientEvent _handle_internal_event(InternalEvent_t event);

void helium_enable_tcxo(){
  SX1276Write( REG_OCP, ( RF_OCP_ON | RF_OCP_TRIM_120_MA ) );
  SX1276Write( REG_TCXO, ( SX1276Read( REG_TCXO ) & RF_TCXO_TCXOINPUT_MASK ) | RF_TCXO_TCXOINPUT_ON );
  SX1276Write( REG_OPMODE, ( SX1276Read( REG_OPMODE ) & RF_OPMODE_MASK ) | RF_OPMODE_SLEEP );
}

void helium_rf_init(struct RfConfig config) {


  // save config in static struct
  LongFi.config = config;

  // configure sx1276 radio events with local helium functions
  LongFi.radio_events.TxDone = OnTxDone;
  LongFi.radio_events.RxDone = OnRxDone;
  LongFi.radio_events.TxTimeout = OnTxTimeout;
  LongFi.radio_events.RxTimeout = OnRxTimeout;
  LongFi.radio_events.RxError = OnRxError;

  // this function calls TimerInits and SX1276IoIrqInit, which are implemented here
  SX1276Init( &LongFi.radio_events );

  SX1276SetChannel( frequency_table[0] );

  SX1276SetTxConfig( MODEM_LORA, TX_OUTPUT_POWER, 0, LORA_BANDWIDTH,
                                 LORA_SPREADING_FACTOR, LORA_CODINGRATE,
                                 LORA_PREAMBLE_LENGTH, LORA_FIX_LENGTH_PAYLOAD_ON,
                                 true, 0, 0, LORA_IQ_INVERSION_ON, 3000 );
  
  SX1276SetRxConfig( MODEM_LORA, LORA_BANDWIDTH, LORA_SPREADING_FACTOR,
                                 LORA_CODINGRATE, 0, LORA_PREAMBLE_LENGTH,
                                 LORA_SYMBOL_TIMEOUT, LORA_FIX_LENGTH_PAYLOAD_ON,
                                 0, true, 0, 0, LORA_IQ_INVERSION_ON, true );
}

void helium_set_buf(uint8_t * buffer, size_t buffer_len){
  LongFi.buffer = buffer;
  LongFi.buffer_len = (buffer!=NULL) ? buffer_len : 0;
}

void helium_rx(){
  SX1276SetRx(0);
}

#define MIN(x,y) ({ \
    typeof(x) _x = (x);     \
    typeof(y) _y = (y);     \
    (void) (&_x == &_y);    \
    _x < _y ? _x : _y; })

void helium_send(const uint8_t * data, size_t len){
  // casting is fine for now, but later we need to spread the packet over channels 
  uint8_t send_len = (uint8_t) MIN(len, LongFi.buffer_len);
   
  for(uint8_t i = 0; i < send_len; i++ )
  {
     LongFi.buffer[i] = data[i];
  }
  SX1276Send(LongFi.buffer, send_len);
}

RxPacket helium_get_rx(){
  RxPacket rx = {
    .buf = LongFi.buffer,
    .len = LongFi.rx_len,
    .rssi = RssiValue,
    .snr = SnrValue,
  };
  LongFi.buffer = NULL;
  LongFi.buffer_len = 0;
  return rx;
}

ClientEvent helium_rf_handle_event(RfEvent event){

  LongFi.cur_event = InternalEvent_None;

  switch (event) {
    case DIO0:
      (*LongFi.dio_irq_handles[0])();
      break;
    case DIO1:
      (*LongFi.dio_irq_handles[1])();
      break;
    case DIO2:
      (*LongFi.dio_irq_handles[2])();
      break;
    case DIO3:
      (*LongFi.dio_irq_handles[3])();
      break;
    case DIO4:
      (*LongFi.dio_irq_handles[4])();
      break;
    case DIO5:
      (*LongFi.dio_irq_handles[5])();
      break;
    case Timer1:
      // TODO: needs to dispatch the callback stashed from TimerInit
      break;
    case Timer2:
      // TODO: needs to dispatch the callback stashed from TimerInit
      break;
    case Timer3:
      // TODO: needs to dispatch the callback stashed from TimerInit
      break;
  }

  return _handle_internal_event(LongFi.cur_event);

}

ClientEvent _handle_internal_event(InternalEvent_t event){
  switch (event) {
    case InternalEvent_None:
      return ClientEvent_None;
      break;
    case InternalEvent_TxDone:
      // could trigger pending Tx'es here
      return ClientEvent_TxDone;
      break;
    case InternalEvent_RxDone:
      // could trigger pending Tx'es here
      return ClientEvent_Rx;
      break;
    case InternalEvent_TxTimeout:
      // potential internal retry logic
      break;
    case InternalEvent_RxTimeout:
      // potential internal retry logic
      break;
    case InternalEvent_RxError:
      // potential internal retry logic
      break;
  }
  return ClientEvent_None;
}

void SX1276IoIrqInit(irq_ptr irq_handlers[NUM_IRQ_HANDLES]){
  for(uint32_t i=0; i<NUM_IRQ_HANDLES; i++){
    LongFi.dio_irq_handles[i] = irq_handlers[i]; 
  }
}


void OnTxDone( void )
{
    LongFi.cur_event = InternalEvent_TxDone;
}

void OnRxDone( uint8_t *payload, uint16_t size, int16_t rssi, int8_t snr )
{
  LongFi.cur_event = InternalEvent_RxDone;
  uint8_t rx_len = (uint8_t) MIN( (size_t) size, LongFi.buffer_len);
  LongFi.rx_len = rx_len;
  memcpy(LongFi.buffer, payload, rx_len);
  RssiValue = rssi;
  SnrValue = snr;
}

void memcpy1(const uint8_t *dst, const uint8_t *src, uint16_t size){
  memcpy((void*) dst, (void*) src, size);
}

void OnTxTimeout( void )
{
  LongFi.cur_event = InternalEvent_TxTimeout;
}

void OnRxTimeout( void )
{
  LongFi.cur_event = InternalEvent_RxTimeout;
}

void OnRxError( void )
{
  LongFi.cur_event = InternalEvent_RxTimeout;
}


