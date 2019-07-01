#include "board.h"
#include "radio/radio.h"
#include "radio/sx1276/sx1276.h"
#include "helium.h"

#ifdef __cplusplus
extern "C" {
#endif


#define RADIO_1                                     916600000
#define RADIO_2                                     920600000
#define FREQ_SPACING                                   200000
#define LONGFI_NUM_UPLINK_CHANNELS                          8

const uint32_t frequency_table[LONGFI_NUM_UPLINK_CHANNELS] = {
  RADIO_1 - FREQ_SPACING*2,
  RADIO_1 - FREQ_SPACING,
  RADIO_1,
  RADIO_2 - FREQ_SPACING*2,
  RADIO_2 - FREQ_SPACING,
  RADIO_2,
  RADIO_2 + FREQ_SPACING,
  RADIO_2 + FREQ_SPACING*2
};

#define NUM_SF        (3)
typedef enum {
  LongFi_SF7 = 0,
  LongFi_SF8 = 1,
  LongFi_SF9 = 2,
} LongFiSpreading_t;

const uint32_t payload_per_fragment[NUM_SF] = {
  32,32,32
};

const uint32_t fragments_per_channel[NUM_SF] = {
  5, 2, 1
};

#define TX_OUTPUT_POWER                             22        // dBm

#define LORA_BANDWIDTH                              0         // [0: 125 kHz,
                                                              //  1: 250 kHz,
                                                              //  2: 500 kHz,
                                                              //  3: Reserved]
#define LORA_SPREADING_FACTOR                       (9)         // [SF7..SF12]
#define LORA_CODINGRATE                             (1)         // [1: 4/5,
                                                              //  2: 4/6,
                                                              //  3: 4/7,
                                                              //  4: 4/8]
#define LORA_PREAMBLE_LENGTH                        (8)         // Same for Tx and Rx
#define LORA_SYMBOL_TIMEOUT                         (5)         // Symbols
#define LORA_FIX_LENGTH_PAYLOAD_ON                  (false)
#define LORA_IQ_INVERSION_ON                        (false)


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
  uint8_t spreading_factor;
  void (*dio_irq_handles[NUM_IRQ_HANDLES])(void);
  RadioEvents_t radio_events;
  InternalEvent_t cur_event;
  uint8_t * buffer;
  size_t buffer_len;
  uint32_t rx_len;
  uint32_t tx_cnt;
  uint32_t tx_len;
} LongFi_t;

typedef struct {
  uint32_t oui;
  uint16_t device_id;
  uint8_t packet_id;
} packet_header_t;

typedef struct {
  uint8_t packet_id;
  uint8_t packet_num;
} fragment_header_t;

#define RX_TIMEOUT_VALUE                            1000
#define BUFFER_SIZE                                 64 // Define the payload size here

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

/*
 * Private helper for counting bytes
 */
size_t payload_bytes_in_first_fragment();
size_t payload_bytes_in_subsequent_fragments();

#ifdef __cplusplus
}
#endif