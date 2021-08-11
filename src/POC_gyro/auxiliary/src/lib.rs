//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] //  bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln};
pub use cortex_m_rt::entry;
pub use stm32f3xx_hal::{delay::Delay, prelude };

use cortex_m::peripheral::ITM;
use stm32f3xx_hal::{
    prelude::*,
    gpio::gpioa::{PA5, PA6, PA7},
    gpio::gpioe::{PE3},
    gpio::{AF5, PushPull, Output},
    pac::{self, SPI1},
    spi::Spi,
};


use i3g4250d::{ MODE};


type A5 = AF5<PushPull>;
pub type I3G4250D = i3g4250d::I3G4250D<Spi<SPI1,(PA5<A5>, PA6<A5>, PA7<A5>)>, PE3<Output<PushPull>>>;


pub fn init() -> (I3G4250D, Delay, ITM) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);

    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    let sck =  gpioa.pa5.into_af5_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5_push_pull(&mut gpioa.moder,  &mut gpioa.otyper, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);



    let mut nss : PE3<Output<PushPull>> = gpioe.pe3.into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    nss.set_high().unwrap();

    let spi = Spi::spi1(dp.SPI1, (sck, miso, mosi), MODE, 1000.Hz(), clocks, &mut rcc.apb2);

    let delay = Delay::new(cp.SYST, clocks);

    let gyro = I3G4250D::new(spi, nss).unwrap();


    (gyro, delay , cp.ITM)
}
