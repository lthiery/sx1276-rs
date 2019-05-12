use sx1276_sys;


pub struct Sx1276;

impl Sx1276 {
	pub fn read(addr: u8) {
		unsafe {
					sx1276_sys::SX1276Read(addr);
		}
	}

	pub fn reset() {
		unsafe {
			//sx1276_sys::SX1276Reset();
		}
	}

}

