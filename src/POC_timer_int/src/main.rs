#![no_std]
#![no_main]


use panic_itm as _; // panic handler

use cortex_m::{iprintln};
use cortex_m_rt::entry;
use stm32f3xx_hal::{
    prelude::*,
    pac,
    rcc::{ RccExt},
};


mod led_interrupt_timer;

use led_interrupt_timer as lit;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut cp = pac::CorePeripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();

    let gpioe = dp.GPIOE.split(&mut rcc.ahb);


    lit::setup(gpioe);

    let itm = &mut cp.ITM.stim[0];

    iprintln!(itm, "Finishe setting up LIT");
    loop {


    }
}
