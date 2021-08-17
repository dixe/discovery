//! # Wiring
//!
//! - PA4 is pwm output and goes to ESC Yellow
//!
//! - ESC Brown to ground
//!
//! - ESC red to 5v
//!
//! This only works if the board has power from the ESC into 5v.
//! Debugging stil seems to work, it just need the power
//!
//!
//!
//!  Pressing the user btn (blue) switches between high and low.
//! - Low is 1 ms pulse width
//! - High is 2ms pulse width
//!
//!
//! Startup
//! - Init start with low and wait for moter 3 beep then long
//! - Now pressins user btn will start and stop the moter
//!
//! adc1 read y axis of stick,
//! - pin: PA1
//! - stick should have 3v in and common ground


//! Based on <https://github.com/stm32-rs/stm32f3xx-hal/blob/master/examples/pwm.rs>


#![no_std]
#![no_main]

//! Example usage for ADC on STM32F303

use panic_itm as _; // panic handler

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

use stm32f3xx_hal::gpio::gpioa::*;
use stm32f3xx_hal::gpio::gpiob::*;
use stm32f3xx_hal::gpio::*;
use lsm303agr::{self, AccelOutputDataRate, mode::MagOneShot, interface::I2cInterface};

pub type Lsm303agr = lsm303agr::Lsm303agr<I2cInterface<I2c<I2C1,(PB6<AF4<OpenDrain>>, PB7<AF4<OpenDrain>>)>>, MagOneShot>;


mod motor1_led1;
mod reset_button;

#[entry]
fn main() -> ! {

    let mut io_parts = init();
    let mut cp = pac::CorePeripherals::take().unwrap();
    let itm = &mut cp.ITM.stim[0];

    iprintln!(itm, "Started motor1 with pwm duty set to {}", io_parts.motor1.get_duty());

    loop {

        // When pressed do this set pwm low and turn of light
        let motor = &mut io_parts.motor1;
        io_parts.reset_btn.check_reset_press(|| {

            motor.set_min();

            iprintln!(itm, "user btn press, pwm duty set to {}", motor.get_duty());
        });

        let adc1_in1_data: u16 = io_parts.adc.read().expect("Error reading from adc1.");
        //iprintln!(&mut itm, "INPUT ADC  {}", adc1_in1_data);


        if adc1_in1_data > 3500 {

            io_parts.motor1.increment_and_set(1);


            iprintln!(itm, "Increasing duty currentlty {}", io_parts.motor1.get_duty());
        }

        else if adc1_in1_data < 500 {

            io_parts.motor1.decrement_and_set(1);

            //iprintln!(&mut itm, "Decreasing  duty currentlty  {} -- adc in={}", motor1.get_duty(), adc1_in1_data);
        }

        if io_parts.lsm303.accel_status().unwrap().xyz_new_data {
            let data = io_parts.lsm303.accel_data().unwrap();
            iprintln!(itm, "Acceleration: x {} y {} z {}", data.x, data.y, data.z);
        }

    }
}


struct IoParts {
    motor1: motor1_led1::Motor1WithLed1,
    reset_btn: reset_button::ResetButton,
    lsm303: Lsm303agr ,
    adc: Adc1PA1
}

/// Combination of adc1 using PA1 as adc source
struct Adc1PA1 {
    /// adc pin to be read from
    adc_pin: PA1<Analog>,
    /// adc that reads from adc_pin
    adc: adc::Adc<pac::ADC1>
}

impl Adc1PA1 {
    pub fn read(&mut self) -> Result<u16, stm32f3xx_hal::nb::Error<()>> {
        self.adc.read(&mut self.adc_pin)
    }

}

fn init() -> IoParts {
    let mut dp = pac::Peripherals::take().unwrap();
    let mut cp = pac::CorePeripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut itm  = &mut cp.ITM.stim[0];

    let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);


    //
    // PIN SETUP
    //

    // GPIOA
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);


    // Setup pin PA0 as analog pin.
    // This pin is connected to the user button on the stm32f3discovery board.
    let pa0 = gpioa
        .pa0
        .into_input(&mut gpioa.moder);


    // Setup pin pa1 as analog input
    // This pin is the pin the adc1 uses to read from joystick
    let mut adc1_in1_pin = gpioa.pa1.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);



    // Setup pin pa4 as digital output
    // This pin is connected to the motor control and produces the PWM
    // signal that controls the motor
    let pa4 = gpioa
        .pa4
        .into_af2_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);



    // GPIOB
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    // Setup pin PB6 as scl
    // This pin is the clock line in the I2C protocol
    let scl = gpiob.pb6.into_af4_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);

    // Setup pin PB7 as sda
    // This pin is the data line in the I2C protocol
    let sda = gpiob.pb7.into_af4_open_drain(&mut gpiob.moder, &mut gpiob.otyper, &mut gpiob.afrl);



    // GPIOE
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    // Setup pe9 as digital output
    // This pin is the north led and used to indicate that the motor
    // is running
    let led3 = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    // Setup I2C for 400 Khz (400.000 Hz)
    let i2c = I2c::new(dp.I2C1, (scl, sda), 400_000.Hz(), clocks, &mut rcc.apb1);

    // Setup lsm (acc and gyro)

    let mut lsm303 = Lsm303agr::new_with_i2c(i2c);
    lsm303.init().unwrap();
    lsm303.set_accel_odr(AccelOutputDataRate::Hz10).unwrap();


    //SETUP ADC FOR ANALOG STICK
    let mut adc1 = adc::Adc::adc1(
        dp.ADC1,
        // setup clock for adc
        &mut dp.ADC1_2,
        &mut rcc.ahb,
        adc::CkMode::default(),
        clocks);




    let adc1_in1_data: u16 = adc1.read(&mut adc1_in1_pin).expect("Error reading from adc1.");

    iprintln!(&mut itm, "PA1 reads {}", adc1_in1_data);


    // TIM3
    //
    // A four channel general purpose timer that's broadly available

    let (_tim3_ch1, tim3_ch2_nopin, _tim3_ch3, _tim3_ch4) = tim3(
        dp.TIM3,
        65535,
        50.Hz(),
        &clocks,
    );



    let reset_btn = reset_button::ResetButton::new(pa0);


    // each channel can have different duty cycle
    let tim3_ch2 = tim3_ch2_nopin.output_to_pa4(pa4);

    let motor1 = motor1_led1::Motor1WithLed1::new(tim3_ch2, led3);


    IoParts {
        motor1,
        reset_btn,
        lsm303,
        adc: Adc1PA1 {
            adc_pin: adc1_in1_pin,
            adc: adc1

        }
    }
}
