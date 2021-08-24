//! # Acceleration data
//!
//! Total acceleration is mesured in milli gs squared.
//! This is due to the measurement being x,y,z and thus a vector.
//! The length of this vector is the total acceleration. To void doing square roots
//! the squared sum is used.

use cortex_m::{iprintln};
use cortex_m_rt::entry;
use stm32f3xx_hal::{
    adc,
    prelude::*,
    pac::{ self, I2C1},
    flash::FlashExt,
    rcc::{ RccExt},
    pwm::{tim3},
    i2c::I2c
};

pub use lsm303agr::{self, AccelOutputDataRate, MagOutputDataRate, mode::{MagOneShot, MagContinuous}, interface::I2cInterface, Measurement};

use stm32f3xx_hal::gpio::gpioa::*;
use stm32f3xx_hal::gpio::gpiob::*;
use stm32f3xx_hal::gpio::*;

use crate::data_types::*;

pub type Lsm303agr = lsm303agr::Lsm303agr<I2cInterface<I2c<I2C1,(PB6<AF4<OpenDrain>>, PB7<AF4<OpenDrain>>)>>, MagOneShot>;

pub type Lsm303agrInit = lsm303agr::Lsm303agr<I2cInterface<I2c<I2C1,(PB6<AF4<OpenDrain>>, PB7<AF4<OpenDrain>>)>>, MagContinuous>;


pub struct AccelAndCompas {
    lsm303: Lsm303agrInit,
    accel_data: Measurement,
    mag_data: Measurement,
}

impl AccelAndCompas {

    pub fn new(mut lsm303: Lsm303agr) -> Self {

        lsm303.init().unwrap();
        lsm303.set_accel_odr(AccelOutputDataRate::Hz10).unwrap();
        lsm303.set_mag_odr(MagOutputDataRate::Hz10).unwrap();

        let lsm303_init = match lsm303.into_mag_continuous() {
            Ok(lsm) => lsm,
            Err(_) => panic!("Lsm mode change panic")
        };


        AccelAndCompas {
            lsm303: lsm303_init,
            accel_data: Measurement {
                x:0,
                y:0,
                z:0
            },
            mag_data: Measurement {
                x:0,
                y:0,
                z:0
            }
        }
    }


    /// Total acceleration in all direction. Result is the squared sum of accelerations
    pub fn total_acceleration(&mut self) -> i32 {
        self.update_accel();

        self.accel_data.x  * self.accel_data.x + self.accel_data.y  * self.accel_data.y + self.accel_data.z  * self.accel_data.z
    }

    pub fn get_accel_data(&mut self) -> Veci32 {
        self.update_accel();
        Veci32 {
            x: self.accel_data.x,
            y: self.accel_data.y,
            z: self.accel_data.z
        }
    }

    fn update_accel(&mut self) {
        if self.lsm303.accel_status().unwrap().xyz_new_data {
            self.accel_data = self.lsm303.accel_data().unwrap();
        }
    }

    fn update_mag(&mut self) {
        let status = self.lsm303.mag_status().unwrap();
        if status.xyz_new_data {
            self.mag_data = self.lsm303.mag_data().unwrap();
        }
    }

    pub fn get_mag_data(&mut self) -> Veci32 {
        self.update_mag();
        Veci32{
            x: self.mag_data.x,
            y: self.mag_data.y,
            z: self.mag_data.z
        }
    }
}
