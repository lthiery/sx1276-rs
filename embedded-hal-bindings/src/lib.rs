use stm32l0xx_hal as hal;
use stm32l0xx_hal::pac::SPI1;
use stm32l0xx_hal::gpio::gpioa::{PA6, PA7};
use stm32l0xx_hal::gpio::gpiob::PB3;
use stm32l0xx_hal::gpio::{Floating, Input};

use embedded_hal::blocking::spi::Write;
#[repr(C)]
pub struct Spi_s
{	
	// would be nice to figure out how to make this generic, but this will do for now
	//hw: hal::spi::Spi<SPI1, (PB3<Input<Floating>>, PA6<Input<Floating>>, PA7<Input<Floating>>)>,
	spi:  *const std::ffi::c_void,
}


#[no_mangle]
pub extern "C" fn root(s: &Spi_s) {

	let spi: &mut hal::spi::Spi<SPI1, (PB3<Input<Floating>>, PA6<Input<Floating>>, PA7<Input<Floating>>)> = 
		unsafe { &mut *(s.spi as *mut hal::spi::Spi<SPI1, (PB3<Input<Floating>>, PA6<Input<Floating>>, PA7<Input<Floating>>)>) };
    
    spi.write(&[0, 1]).unwrap();
}
