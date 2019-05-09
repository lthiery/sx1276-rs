use stm32l0xx_hal as hal;
use stm32l0xx_hal::pac::SPI1;
use stm32l0xx_hal::gpio::gpioa::{PA6, PA7};
use stm32l0xx_hal::gpio::gpiob::PB3;
use stm32l0xx_hal::gpio::{Floating, Input};


#[repr(C)]
pub struct Spi_s
{	
	// would be nice to figure out how to make this generic, but this will do for now
	spi: hal::spi::Spi<SPI1, (PB3<Input<Floating>>, PA6<Input<Floating>>, PA7<Input<Floating>>)>
}


#[no_mangle]
pub extern "C" fn root(s: Spi_s) {}
