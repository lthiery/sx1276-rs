#![no_std]
#![allow(non_snake_case)]

use stm32l0xx_hal as hal;
use stm32l0xx_hal::gpio::gpioa::{PA6, PA7};
use stm32l0xx_hal::gpio::gpiob::PB3;
use stm32l0xx_hal::gpio::{Floating, Input, Output, PushPull};
use stm32l0xx_hal::pac::SPI1;

//use embedded_hal::blocking::spi::Write;
use embedded_hal::digital::v1::OutputPin;
use embedded_hal::spi::FullDuplex;
use core::ffi; 
use nb::block;


#[repr(C)]
pub struct SpiInstance {
    Instance:*mut ffi::c_void,
    //Reset: &Gpio_t,
}

#[repr(C)]
pub struct Spi_s {
    Spi: SpiInstance,
    Nss: Gpio_t,
}

pub type Spi_t = Spi_s;//


//find more elegant way to make cbindgen export Spi_t
#[no_mangle]
pub extern "C" fn foo(s: Spi_t) {
}

#[no_mangle]
pub extern "C" fn SpiInOut(s: &mut Spi_t, outData: u16) -> u16{

    //let data: &mut State = unsafe { &mut *(data as *mut State) };
    //let data: &mut State = unsafe { &mut *(data as *mut State) };


    let spi: &mut hal::spi::Spi<
        SPI1,
        (
            PB3<Input<Floating>>,
            PA6<Input<Floating>>,
            PA7<Input<Floating>>,
        ),
    > = unsafe {
        &mut *(s.Spi.Instance as *mut hal::spi::Spi<
            SPI1,
            (
                PB3<Input<Floating>>,
                PA6<Input<Floating>>,
                PA7<Input<Floating>>,
            ),
        >)
    };

    let mut inData: u16;

    spi.send((outData >> 8) as u8).unwrap();
    inData = (block!(spi.read()).unwrap() as u16) << 8;

    spi.send(outData as u8).unwrap();
    inData |= block!(spi.read()).unwrap() as u16;

    inData
}

type Gpio_t = *mut usize;

#[repr(C)]
pub enum PinNames {
    MCU_PINS,
    IOE_PINS,
    RADIO_RESET,
}

#[repr(C)]
pub enum PinModes {
    PIN_INPUT = 0,
    PIN_OUTPUT,
    PIN_ALTERNATE_FCT,
    PIN_ANALOGIC,
}

#[repr(C)]
pub enum PinTypes {
    PIN_NO_PULL = 0,
    PIN_PULL_UP,
    PIN_PULL_DOWN,
}

#[repr(C)]
pub enum PinConfigs {
    PIN_PUSH_PULL = 0,
    PIN_OPEN_DRAIN,
}

#[no_mangle]
pub extern "C" fn GpioInit(
    obj: &Gpio_t,
    pin: PinNames,
    mode: PinModes,
    config: PinConfigs,
    pin_type: PinTypes,
    val: u32,
) {

}

#[no_mangle]
pub extern "C" fn GpioWrite(obj: &Gpio_t, val: u8) {
    // let gpio: &mut stm32l0xx_hal::gpio::gpioa::PA15<Output<PushPull>> =
    //     unsafe { &mut *(obj as *mut stm32l0xx_hal::gpio::gpioa::PA15<Output<PushPull>>) };

    // if (val == 0) {
    //     gpio.set_low();
    // } else {
    //     gpio.set_high();
    // }
}

#[repr(C)]
pub struct TimerEvent_s {
    IsRunning: bool,
}

type TimerEvent_t = TimerEvent_s;

#[no_mangle]
pub extern "C" fn TimerInit(obj: &TimerEvent_t, cb: Option<extern "C" fn()>) {}

#[no_mangle]
pub extern "C" fn TimerIrqHandler() {}

#[no_mangle]
pub extern "C" fn TimerStart(obj: &TimerEvent_t) {}

#[no_mangle]
pub extern "C" fn TimerStop(obj: &TimerEvent_t) {}

#[no_mangle]
pub extern "C" fn TimerReset(obj: &TimerEvent_t) {}

#[no_mangle]
pub extern "C" fn TimerSetValue(obj: &TimerEvent_t, value: u32) {}

#[no_mangle]
pub extern "C" fn TimerGetCurrentTime() {}

#[no_mangle]
pub extern "C" fn TimerGetElapsedTime(saved_time: &TimerEvent_t) {}

#[no_mangle]
pub extern "C" fn TimerGetFutureTime(event_in_future: &TimerEvent_t) {}

#[no_mangle]
pub extern "C" fn TimerLowPowerHandler() {}

type irq_ptr = extern "C" fn();


#[no_mangle]
pub extern "C" fn SX1276IoIrqInit(irq_handlers: [irq_ptr; 6]) {}

#[no_mangle]
pub extern "C" fn SX1276GetPaSelect(channel: u32) -> u8 {0}

#[no_mangle]
pub extern "C" fn DelayMs(ms: u32){}

#[no_mangle]
pub extern "C" fn memcpy1(dst: &u8, src: &u8, size: u16){}

#[no_mangle]
pub extern "C" fn SX1276SetAntSwLowPower(status: bool){}

#[no_mangle]
pub extern "C" fn SX1276SetAntSw(rxTx: u8){}

#[no_mangle]
pub extern "C" fn assert_param(expr: bool){}