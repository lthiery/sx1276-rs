use stm32l0xx_hal as hal;
use stm32l0xx_hal::pac::SPI1;
use stm32l0xx_hal::gpio::gpioa::{PA6, PA7};
use stm32l0xx_hal::gpio::gpiob::PB3;
use stm32l0xx_hal::gpio::{Floating, Input};

use embedded_hal::blocking::spi::Write;

#[repr(C)]
#[no_mangle]
pub struct spi_handle
{	
	spi:  *mut std::ffi::c_void,
}

type Spi_t = spi_handle;

// #[repr(C)]
// #[no_mangle]
// pub struct pin_handle
// {	
// 	name: u8,
// }

// #[no_mangle]
// pub extern "C" fn SpiInit(s: &mut Spi_s, _mosi: PinNames, _miso: PinNames, _sclk: PinNames, _nss: PinNames) {
// 	let spi: &mut hal::spi::Spi<SPI1, (PB3<Input<Floating>>, PA6<Input<Floating>>, PA7<Input<Floating>>)> = 
// 		unsafe { &mut *(s.spi as *mut hal::spi::Spi<SPI1, (PB3<Input<Floating>>, PA6<Input<Floating>>, PA7<Input<Floating>>)>) };
    
//     spi.write(&[0, 1]).unwrap();
// }
#[no_mangle]
pub extern "C" fn SpiDeInit(s: &mut Spi_t){}
#[no_mangle]
pub extern "C" fn SpiFormat(s: &mut Spi_t, bits: u8, cpol: u8, cpha: u8, slave: u8){}
#[no_mangle]
pub extern "C" fn SpiFrequency(s: &mut Spi_t, hz: u8){}
#[no_mangle]
pub extern "C" fn SpiInOut(s: &mut Spi_t, outData: u16){}

#[repr(C)]
#[no_mangle]
pub struct gpio_handle
{	
	gpio:  *mut std::ffi::c_void,
}

type Gpio_t = spi_handle;

pub enum PinNames
{
    MCU_PINS,
    IOE_PINS,
    NC = 0xFFFFFFFF
}

pub enum PinModes
{
    PIN_INPUT = 0,
    PIN_OUTPUT,
    PIN_ALTERNATE_FCT,
    PIN_ANALOGIC
}

pub enum PinTypes
{
    PIN_NO_PULL = 0,
    PIN_PULL_UP,
    PIN_PULL_DOWN
}

pub enum PinConfigs
{
    PIN_PUSH_PULL = 0,
    PIN_OPEN_DRAIN
}



#[no_mangle]
pub extern "C" fn GpioInit(obj: &mut Gpio_t, pin: PinNames, mode: PinModes, config: PinConfigs, pin_type: PinTypes, val: u32){

}

