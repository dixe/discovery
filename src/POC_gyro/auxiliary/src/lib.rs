//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] //  bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln};
pub use cortex_m_rt::entry;
pub use stm32f3xx_hal::{delay::Delay, prelude, SPI1, spi1 };

use cortex_m::peripheral::ITM;
use stm32f3xx_hal::{
    prelude::*,
    gpio::gpiob::{PB6, PB7},
    gpio::{AF4},
    pac::{self, spi1,},

};


use i3g4250d::I3G4250D;

pub fn init() -> (&'static i2c1::RegisterBlock, I3g4250d, Delay, ITM) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let mut miso = gpioa.pa4.into_af5_push_pull(&mut gpioa.moder,  &mut gpioa.otyper, &mut gpioa.afrl);

    let mut mosi = gpioa.pa7.into_af5_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);

    let mut sck =  gpioa.pa5.into_af5_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);


    let spi = 2;
    let delay = Delay::new(cp.SYST, clocks);

    let mut gyro = i3g4250d::I3G4250D::new(spi);


    unsafe { (&mut *(I2C1::ptr() as *mut _), gyro, delay, cp.ITM) }
}
