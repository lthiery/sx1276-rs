#include <stdint.h>
#include <stdlib.h>
#include <stdbool.h>


// I'm calling "mail" downstream packets
// Presumably, our RF ACK provide a bit-flag notice about mail being available
typedef enum QualityOfService {
	QOS_0, // YOLO packets out and go to sleep. RX only during network_poll
	QOS_1, // make best effort to get ACKs, provide client notice of mail being available
	QOS_2, // make best effort to get ACKs and pull down mail ASAP
} QualityOfService;

struct RfConfig {
	bool always_on;
	// or be able to asyncronously rx at anytime (this would be Class A vs Class B in LoRa or something)
	QualityOfService qos;
	uint32_t network_poll; // even w/o data, protocol could be configured to poll the network periodically
};

void helium_rf_init(struct RfConfig);

typedef enum ClientEvent {
	ClientEvent_None,
	ClientEvent_TxDone,
	ClientEvent_Rx,
} ClientEvent;

// this is an interrupt safe call that pushes the event into a queue inside the protocol library
bool helium_rf_queue_event(ClientEvent);

// this will give ownership of a buffer to helium_rf
// should it trigger automatic fetch of mail if it remembers it from previous ACK?
// if no, then we need to provide an API for client to do that action specifically
void helium_rf_set_rx_buf(uint8_t * buf, uint16_t size);

typedef enum RfEvent {
	DIO0,
	DIO1,
	DIO2,
	DIO3,
	DIO4,
	DIO5,
	Timer1,
	Timer2,
	Timer3
} RfEvent;

// to be used by client to loop over process_event
ClientEvent helium_rf_handle_event(RfEvent);