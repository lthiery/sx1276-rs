use sx1276_sys;


pub struct Sx1276;


// typedef struct
// {
//     /*!
//      * \brief  Tx Done callback prototype.
//      */
//     void    ( *TxDone )( void );
//     /*!
//      * \brief  Tx Timeout callback prototype.
//      */
//     void    ( *TxTimeout )( void );
//     /*!
//      * \brief Rx Done callback prototype.
//      *
//      * \param [IN] payload Received buffer pointer
//      * \param [IN] size    Received buffer size
//      * \param [IN] rssi    RSSI value computed while receiving the frame [dBm]
//      * \param [IN] snr     Raw SNR value given by the radio hardware
//      *                     FSK : N/A ( set to 0 )
//      *                     LoRa: SNR value in dB
//      */
//     void    ( *RxDone )( uint8_t *payload, uint16_t size, int16_t rssi, int8_t snr );
//     /*!
//      * \brief  Rx Timeout callback prototype.
//      */
//     void    ( *RxTimeout )( void );
//     /*!
//      * \brief Rx Error callback prototype.
//      */
//     void    ( *RxError )( void );
//     /*!
//      * \brief  FHSS Change Channel callback prototype.
//      *
//      * \param [IN] currentChannel   Index number of the current channel
//      */
//     void ( *FhssChangeChannel )( uint8_t currentChannel );

//     /*!
//      * \brief CAD Done callback prototype.
//      *
//      * \param [IN] channelDetected    Channel Activity detected during the CAD
//      */
//     void ( *CadDone ) ( bool channelActivityDetected );
// }RadioEvents_t;

#[no_mangle]
pub extern "C" fn tx_done() {

}

#[no_mangle]
pub extern "C" fn tx_timeout() {
	
}

#[no_mangle]
pub extern "C" fn rx_done(payload: *mut  u8, size: u16, rssi: i16, snr: i8) {

}

#[no_mangle]
pub extern "C" fn rx_timeout() {
	
}

#[no_mangle]
pub extern "C" fn rx_error() {
	
}

#[no_mangle]
pub extern "C" fn fhss_change_channel(current_channel: u8) {
	
}

#[no_mangle]
pub extern "C" fn cad_done(channel_activity_detected: bool) {
	
}


impl Sx1276 {
	pub fn read(addr: u8) {
		unsafe {
			sx1276_sys::SX1276Read(addr);
		}
	}

	pub fn initialize() {
		let mut radio_events = sx1276_sys::RadioEvents_t {
			TxDone: Some(tx_done),
			TxTimeout: Some(tx_timeout),
			RxDone: Some(rx_done),
			RxTimeout: Some(rx_timeout),
			RxError: Some(rx_error),
			FhssChangeChannel: Some(fhss_change_channel),
			CadDone: Some(cad_done)
		};
		unsafe {
			sx1276_sys::SX1276Init(&mut radio_events);
		}
	}

}

