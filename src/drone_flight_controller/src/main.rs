

#![no_std]
#![no_main]

//! Example usage for ADC on STM32F303

use panic_itm as _; // panic handler

use cortex_m::{iprintln, peripheral::itm::Stim};
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


mod io_parts;
mod accel_and_compas;
mod motor1_led1;
mod reset_button;
mod data_types;
#[entry]
fn main() -> ! {

    let mut io_parts = io_parts::init();
    let mut cp = pac::CorePeripherals::take().unwrap();
    let itm = &mut cp.ITM.stim[0];


    let mut accel_info = DataInformation::new();

    let baseline = 1000.0 * 1000.0 ;
    loop {
        update_and_print_accel_info(&mut io_parts.accel_and_compas, &mut io_parts.reset_btn, &mut accel_info, itm);
        //update_and_print_mag_info(&mut io_parts.accel_and_compas, itm);

    }
}


fn update_and_print_accel_info(lsm303: &mut accel_and_compas::AccelAndCompas,
                               reset_btn: &mut reset_button::ResetButton,
                               accel_info: &mut DataInformation,
                               itm: &mut Stim) {
    // could also use subtract to make default earth acceleration give measure close to 0
    // This could avoid division which might be better for performance

    let baseline = 1000.0 * 1000.0 ;

    let new_measure = (lsm303.total_acceleration() as f32) / baseline;

    accel_info.add_measure(new_measure);

    //iprintln!(itm, "min: {} max: {}", accel_info.min(), accel_info.max());

    let data = lsm303.get_accel_data();

    iprintln!(itm, "x {} y {} z {}", data.x, data.y, data.z);


    // reset if pressed
    reset_btn.check_reset_press(|| {

        accel_info.reset_min_max();

    });

}

fn update_and_print_mag_info(lsm303: &mut accel_and_compas::AccelAndCompas, itm: &mut Stim) {

    let data = lsm303.get_mag_data();

    iprintln!(itm, "x {} y {} z {}", data.x, data.y, data.z);

}


struct DataInformation {
    index: usize,
    data: [f32; 20],
    average: f32,
    min: f32,
    max: f32
}

impl DataInformation {

    pub fn new() -> Self {

        Self {
            index: 0,
            data: [0.0; 20],
            average: 0.0,
            min: f32::MAX,
            max: f32::MIN
        }
    }

    pub fn add_measure(&mut self, data: f32) {
        self.data[self.index] = data;
        self.index = (self.index + 1) % 20;
        self.update();
    }

    pub fn average(&self) -> f32 {
        self.average
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn min(&self) -> f32 {
        self.min
    }


    pub fn reset_min_max(&mut self) {
        self.min = self.average;
        self.max = self.average;
    }


    fn update(&mut self)  {
        self.average = 0.0;
        for i in 0..20 {
            self.average += self.data[i];
        }

        self.average = self.average / 20.0;
        self.max = f32::max(self.max, self.average);
        self.min = f32::min(self.min, self.average);
    }
}
