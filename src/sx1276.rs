#![no_std]
#![no_main]
#![feature(lang_items)]

extern crate panic_halt;
use stm32l0xx_hal as hal;
use sx1276;

use sx1276::{RfConfig, QualityOfService, ClientEvent, RfEvent};
use sx1276::LongFi;

use core::fmt::Write;
use hal::{exti::TriggerEdge, gpio::*, pac, prelude::*, rcc::Config, spi, serial};
use hal::serial::USART2;



use embedded_hal::digital::v2::OutputPin;

#[rtfm::app(device = stm32l0xx_hal::pac)]
const APP: () = {

    #[init(resources = [BUFFER])]
    fn init() {
    }

};

#![no_std]
use sx1276_sys;

pub use sx1276_sys::QualityOfService;
pub use sx1276_sys::RfConfig;
pub use sx1276_sys::RxPacket;
pub use sx1276_sys::RfEvent;
pub use sx1276_sys::ClientEvent;

pub struct LongFi;

impl LongFi{

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

    pub fn get_rx() -> RxPacket {
        unsafe {
            sx1276_sys::helium_get_rx()
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

