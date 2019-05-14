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


impl<SPI, NSS> Sx1276<SPI, NSS>{
	pub fn new(spi: SPI, nss: NSS) -> Sx1276<SPI, NSS>{
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

