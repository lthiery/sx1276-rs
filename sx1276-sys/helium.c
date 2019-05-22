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

#define RX_TIMEOUT_VALUE                            1000
#define BUFFER_SIZE                                 64 // Define the payload size here

const uint8_t PingMsg[] = "PING";
const uint8_t PongMsg[] = "PONG";

uint16_t BufferSize = BUFFER_SIZE;
uint8_t Buffer[BUFFER_SIZE];


int8_t RssiValue = 0;
int8_t SnrValue = 0;

/*!
 * Radio events function pointer
 */
static RadioEvents_t RadioEvents;

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
    RadioEvents.TxDone = OnTxDone;
    RadioEvents.RxDone = OnRxDone;
    RadioEvents.TxTimeout = OnTxTimeout;
    RadioEvents.RxTimeout = OnRxTimeout;
    RadioEvents.RxError = OnRxError;

    SX1276Init( &RadioEvents );

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
  // switch (event) {
  //   case None,
  // TxDone,
  // TxDoneAndRxPending,
  // TxDoneAndRx,
  // NewRx
  // }
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


