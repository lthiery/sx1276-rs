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


pub use sx1276_sys::QualityOfService;
pub use sx1276_sys::RfConfig;

pub use sx1276_sys::RfEvent;
pub use sx1276_sys::ClientEvent;

pub struct LongFi;

impl LongFi{


	pub fn initialize(config: RfConfig) {
		unsafe{
			sx1276_sys::helium_rf_init(config);
		}
	}

	//for debugging
	pub fn raw_read(addr: u8) -> u8 {
		unsafe {
			sx1276_sys::SX1276Read(addr)
		}
	}

	//for debugging
	pub fn set_rx() {
		unsafe {
			sx1276_sys::SX1276SetRx(0);
		}
	}

	pub fn handle_event(event: RfEvent) -> ClientEvent {
		unsafe {
			sx1276_sys::helium_rf_handle_event(event)
		}
	}

	pub fn send_ping() {
		unsafe {
			sx1276_sys::helium_ping();
		}
	}

	pub fn send_pong() {
		unsafe {
			sx1276_sys::helium_pong();
		}
	}

}

