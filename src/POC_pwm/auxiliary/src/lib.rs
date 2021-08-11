
//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::asm::bkpt;
pub use cortex_m_rt::entry;
pub use f3::hal::stm32f30x::{gpioc, rcc, adc1};

use f3::hal::stm32f30x::{self, GPIOE, RCC, ADC3};

pub fn init() -> (&'static gpioc::RegisterBlock, &'static rcc::RegisterBlock, &'static adc1::RegisterBlock ) {
    // restrict access to the other peripherals
    (stm32f30x::Peripherals::take().unwrap());

    unsafe { (&*GPIOE::ptr(), &*RCC::ptr(), &*ADC3::ptr()) }
}
