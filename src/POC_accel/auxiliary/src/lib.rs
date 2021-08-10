//! Initialization code

#![no_std]

#[allow(unused_extern_crates)] //  bug rust-lang/rust#53964
extern crate panic_itm; // panic handler

pub use cortex_m::{asm::bkpt, iprint, iprintln};
pub use cortex_m_rt::entry;
pub use stm32f3_discovery::stm32f3xx_hal::{delay::Delay, prelude, stm32::i2c1};

use cortex_m::peripheral::ITM;
use stm32f3_discovery::{
    stm32f3xx_hal::{
        i2c::I2c,
        prelude::*,
        stm32::{self, I2C1},
        gpio::gpiob::{PB6, PB7},
        gpio::{AF4},


    },
};

use lsm303agr::{self, AccelOutputDataRate, mode::MagOneShot, interface::I2cInterface};

pub type Lsm303agr = lsm303agr::Lsm303agr<I2cInterface<I2c<I2C1,(PB6<AF4>, PB7<AF4>)>>, MagOneShot>;


pub fn init() -> (&'static i2c1::RegisterBlock, Lsm303agr, Delay, ITM) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let i2c = I2c::new(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);

    let delay = Delay::new(cp.SYST, clocks);

    let mut sensor = lsm303agr::Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz10).unwrap();


    unsafe { (&mut *(I2C1::ptr() as *mut _), sensor,delay, cp.ITM) }
}
