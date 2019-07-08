#![no_std]
use sx1276_sys;

pub use sx1276_sys::QualityOfService;
pub use sx1276_sys::RfConfig;
pub use sx1276_sys::RxPacket;
pub use sx1276_sys::RfEvent;
pub use sx1276_sys::ClientEvent;

pub struct LongFi;

impl LongFi{

	pub fn enable_tcxo() {
		unsafe{
			sx1276_sys::helium_enable_tcxo();
		}
	}

	pub fn initialize(config: RfConfig) {
		unsafe{
			sx1276_sys::helium_rf_init(config);
		}
	}

	pub fn set_buffer(buffer: &mut [u8]) {
		unsafe {
			sx1276_sys::helium_set_buf(buffer.as_mut_ptr(), buffer.len());
		}
	}

	//for debugging
	pub fn raw_read(addr: u8) -> u8 {
		unsafe {
			sx1276_sys::SX1276Read(addr)
		}
	}

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

	pub fn send(buffer: &[u8], len: usize){
		let send_len = ::core::cmp::min(len, buffer.len());
		unsafe {
			sx1276_sys::helium_send(buffer.as_ptr(), send_len);
		}
	}

	pub fn send_test(){
		unsafe {
			sx1276_sys::helium_rf_test();
		}
	}

	pub fn get_rx() -> RxPacket {
		unsafe {
			sx1276_sys::helium_get_rx()
		}
	}

	pub fn get_random() -> u32 {
		unsafe {
			sx1276_sys::SX1276Random()
		}
	}
}

extern crate libm;

#[no_mangle]
pub extern "C" fn ceil(expr: f64) -> f64 {
	libm::ceil(expr)
}

#[no_mangle]
pub extern "C" fn round(expr: f64) -> f64 {
	libm::round(expr)
}

#[no_mangle]
pub extern "C" fn floor(expr: f64) -> f64 {
	libm::floor(expr)
}
