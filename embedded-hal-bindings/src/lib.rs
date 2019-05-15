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
pub struct Spi_s {
    s: *mut ffi::c_void,
}

type Spi_t = Spi_s;//

#[no_mangle]
pub extern "C" fn SpiInOut(s: Spi_t, outData: u16) {

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
        &mut *(s.s as *mut hal::spi::Spi<
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
}

type Gpio_t = *mut u32;

#[repr(C)]
pub enum PinNames {
    MCU_PINS,
    IOE_PINS,
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
    obj: Gpio_t,
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
