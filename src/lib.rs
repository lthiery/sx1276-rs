#![no_std]
use sx1276_sys;

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

pub struct Sx1276<SPI, NSS>{
	spi: SPI,
	nss: NSS
}

//903.5


impl<SPI, NSS> Sx1276<SPI, NSS>{
	pub fn new(spi: SPI, nss: NSS) -> Sx1276<SPI, NSS>{
		
		Self::initialize();

		Sx1276 {
			spi,
			nss
		}
	}

	pub fn read(&self, addr: u8) -> u8 {
		unsafe {
			sx1276_sys::SX1276Read(addr)
		}
	}

	fn initialize() {
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
	// 911_000_000 - 400_000
	pub fn set_channel(&self, freq: u32){
		unsafe {
			sx1276_sys::SX1276SetChannel(freq);
		}
	}

	/*
	#if USE_MODEM_LORA == 1
 
    #define LORA_BANDWIDTH                              2         // [0: 125 kHz,
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
    #define LORA_FHSS_ENABLED                           false  
    #define LORA_NB_SYMB_HOP                            4     
    #define LORA_IQ_INVERSION_ON                        false
    #define LORA_CRC_ENABLED                            true

	*/

	pub fn set_tx_config(&self){
		unsafe {
			sx1276_sys::SX1276SetTxConfig(
				sx1276_sys::RadioModems_t_MODEM_LORA,
				14, //output power
				0, // fdev
				2, //bandwidth
				1, //datarate FSK only?
				1, //coderate
				0, //preamble len
				false, //fixlen
				false,
				false,
				0,
				false,
				500
			);
		}
	}

	pub fn boop(&self){
		unsafe {
			let mut packet: [u8; 4] = [ 0xDE, 0xAD, 0xBE, 0xEF];
			sx1276_sys::SX1276Send(packet.as_mut_ptr(), packet.len() as u8);

		}



	}


}

