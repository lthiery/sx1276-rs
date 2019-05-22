#include "helium.h"
#include "board.h"

#include "radio/radio.h"
#include "radio/sx1276/sx1276.h"

#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>


#define RF_FREQUENCY                                911200000 // Hz //911_000_000 - 400_000
#define TX_OUTPUT_POWER                             14        // dBm

#define LORA_BANDWIDTH                              0         // [0: 125 kHz,
                                                              //  1: 250 kHz,
                                                              //  2: 500 kHz,
                                                              //  3: Reserved]
#define LORA_SPREADING_FACTOR                       7         // [SF7..SF12]
#define LORA_CODINGRATE                             1         // [1: 4/5,
                                                              //  2: 4/6,
                                                              //  3: 4/7,
                                                              //  4: 4/8]
#define LORA_PREAMBLE_LENGTH                        8         // Same for Tx and Rx
#define LORA_SYMBOL_TIMEOUT                         5         // Symbols
#define LORA_FIX_LENGTH_PAYLOAD_ON                  false
#define LORA_IQ_INVERSION_ON                        false

typedef enum
{
    LOWPOWER,
    RX,
    RX_TIMEOUT,
    RX_ERROR,
    TX,
    TX_TIMEOUT,
} States_t;

#define NUM_IRQ_HANDLES   6

typedef struct {
    void * dio_irq_handles[NUM_IRQ_HANDLES];
    RadioEvents_t radio_events;
} LongFi_t;

#define RX_TIMEOUT_VALUE                            1000
#define BUFFER_SIZE                                 64 // Define the payload size here

const uint8_t PingMsg[] = "PING";
const uint8_t PongMsg[] = "PONG";

uint16_t BufferSize = BUFFER_SIZE;
uint8_t Buffer[BUFFER_SIZE];


int8_t RssiValue = 0;
int8_t SnrValue = 0;

/*!
 * Given the constraints of the SX1276 API, we have to use static struct
 * I guess it's fine since we are unlikely to ever need more than one instance
 */
static LongFi_t LongFi;

/*!
 * \brief Function to be executed on Radio Tx Done event
 */
void OnTxDone( void );

/*!
 * \brief Function to be executed on Radio Rx Done event
 */
void OnRxDone( uint8_t *payload, uint16_t size, int16_t rssi, int8_t snr );

/*!
 * \brief Function executed on Radio Tx Timeout event
 */
void OnTxTimeout( void );

/*!
 * \brief Function executed on Radio Rx Timeout event
 */
void OnRxTimeout( void );

/*!
 * \brief Function executed on Radio Rx Error event
 */
void OnRxError( void );


void helium_rf_init(struct RfConfig config) {
    // Radio initialization
    LongFi.radio_events.TxDone = OnTxDone;
    LongFi.radio_events.RxDone = OnRxDone;
    LongFi.radio_events.TxTimeout = OnTxTimeout;
    LongFi.radio_events.RxTimeout = OnRxTimeout;
    LongFi.radio_events.RxError = OnRxError;

    SX1276Init( &LongFi.radio_events );

    SX1276SetChannel( RF_FREQUENCY );

    SX1276SetTxConfig( MODEM_LORA, TX_OUTPUT_POWER, 0, LORA_BANDWIDTH,
                                   LORA_SPREADING_FACTOR, LORA_CODINGRATE,
                                   LORA_PREAMBLE_LENGTH, LORA_FIX_LENGTH_PAYLOAD_ON,
                                   true, 0, 0, LORA_IQ_INVERSION_ON, 3000 );
    
    SX1276SetRxConfig( MODEM_LORA, LORA_BANDWIDTH, LORA_SPREADING_FACTOR,
                                   LORA_CODINGRATE, 0, LORA_PREAMBLE_LENGTH,
                                   LORA_SYMBOL_TIMEOUT, LORA_FIX_LENGTH_PAYLOAD_ON,
                                   0, true, 0, 0, LORA_IQ_INVERSION_ON, true );
}

ClientEvent helium_rf_handle_event(RfEvent event){

  switch (event) {
    case DIO0:
      LongFi.dio_irq_handles[0];
      break;
    case DIO1:
      LongFi.dio_irq_handles[1];
      break;
    case DIO2:
      LongFi.dio_irq_handles[2];
      break;
    case DIO3:
      LongFi.dio_irq_handles[3];
      break;
    case DIO4:
      LongFi.dio_irq_handles[4];
      break;
    case DIO5:
      LongFi.dio_irq_handles[5];
      break;
    case Timer1:
    break;
    case Timer2:
    break;
    case Timer3:
    break;
  }

}

void SX1276IoIrqInit(irq_ptr irq_handlers[NUM_IRQ_HANDLES]){
  for(uint32_t i=0; i<NUM_IRQ_HANDLES; i++){
    LongFi.dio_irq_handles[i] = irq_handlers[i]; 
  }
  
}



// // this is an interrupt safe call that pushes the event into a queue inside the protocol library
// bool helium_rf_queue_event(ClientEvent_t){

// }

// // this will give ownership of a buffer to helium_rf
// // should it trigger automatic fetch of mail if it remembers it from previous ACK?
// // if no, then we need to provide an API for client to do that action specifically
// void helium_rf_set_rx_buf(uint8_t * buf, uint16_t size){

// }

// // to be used by client to loop over process_event
// bool helium_rf_has_events(){

// }
// ClientEvent_t helium_rf_process_event(){

// }


void OnTxDone( void )
{
    //SX1276Sleep( );
}

void OnRxDone( uint8_t *payload, uint16_t size, int16_t rssi, int8_t snr )
{
    //SX1276Sleep( );
    BufferSize = size;
    //memcpy( Buffer, payload, BufferSize );
    RssiValue = rssi;
    SnrValue = snr;
}

void OnTxTimeout( void )
{
    //SX1276Sleep( );
}

void OnRxTimeout( void )
{
    //SX1276Sleep( );
}

void OnRxError( void )
{
    //SX1276Sleep( );
}


