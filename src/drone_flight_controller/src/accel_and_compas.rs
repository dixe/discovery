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

pub use lsm303agr::{self, AccelOutputDataRate, mode::MagOneShot, interface::I2cInterface, Measurement};

use stm32f3xx_hal::gpio::gpioa::*;
use stm32f3xx_hal::gpio::gpiob::*;
use stm32f3xx_hal::gpio::*;

use crate::data_types::*;

pub type Lsm303agr = lsm303agr::Lsm303agr<I2cInterface<I2c<I2C1,(PB6<AF4<OpenDrain>>, PB7<AF4<OpenDrain>>)>>, MagOneShot>;


pub struct AccelAndCompas {
    lsm303: Lsm303agr,
    accel_data: Measurement,
    mag_data: Measurement,
}

impl AccelAndCompas {

    pub fn new(lsm303: Lsm303agr) -> Self {
        AccelAndCompas {
            lsm303,
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

    fn update_accel(&mut self) {
        if self.lsm303.accel_status().unwrap().xyz_new_data {
            self.accel_data = self.lsm303.accel_data().unwrap();
        }
    }

    fn update_mag(&mut self) {
        if self.lsm303.mag_status().unwrap().xyz_new_data {
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
