#![no_std]
#![no_main]

//! Example usage for ADC on STM32F303

use panic_itm as _; // panic handler

use cortex_m::{iprintln};
use cortex_m_rt::entry;
use stm32f3xx_hal::{
    adc,
    prelude::*,
    pac,
    flash::FlashExt,
    rcc::{ RccExt},
    gpio::{self},
    pwm::{self, tim3},
};


mod reset_button;


// Wiring
// PA4 is pwm output and goes to ESC Yellow
// ESC Brown to ground
// ESC red to 5v

// This only works if the board has power from the ESC into 5v
// Debugging stil seems to work, it just need the power

// Pressing the user btn (blue) switches between high and low
// low is 1 ms pulse width
// hiigh is 2ms pulse width

// Init start with low and wait for moter 3 beep then long
// Now pressins user btn will start and stop the moter


// adc1 read y axis of stick,
// pin: PA1
// stick should have 3v in and common ground


// Based on https://github.com/stm32-rs/stm32f3xx-hal/blob/master/examples/pwm.rs
#[entry]
fn main() -> ! {

    let mut dp = pac::Peripherals::take().unwrap();

    let mut cp = pac::CorePeripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    let mut itm = &mut cp.ITM.stim[0];

    let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);


    let mut adc1 = adc::Adc::adc1(
        dp.ADC1,
        // setup clock for adc
        &mut dp.ADC1_2,
        &mut rcc.ahb,
        adc::CkMode::default(),
        clocks);


    // Set up pin PA0 as analog pin.
    // This pin is connected to the user button on the stm32f3discovery board.
    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);



    let mut adc1_in1_pin = gpioa.pa1.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);

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


    let led3 = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let pa4 = gpioa
        .pa4
        .into_af2_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);


    let pa0 = gpioa
        .pa0
        .into_input(&mut gpioa.moder);


    let mut reset_btn = reset_button::ResetButton::new(pa0);


    // each channel can have different duty cycle
    let tim3_ch2 = tim3_ch2_nopin.output_to_pa4(pa4);

    let mut motor1 = Motor1WithLed1::new(tim3_ch2, led3);
    iprintln!(&mut itm, "Started motor1 with pwm duty set to {}", motor1.get_duty());


    loop {


        // When pressed do this set pwm low and turn of light
        reset_btn.check_reset_press(|| {

            motor1.set_min();
            iprintln!(&mut itm, "user btn press, pwm duty set to {}", motor1.get_duty());
        });

        let adc1_in1_data: u16 = adc1.read(&mut adc1_in1_pin).expect("Error reading from adc1.");
        //iprintln!(&mut itm, "INPUT ADC  {}", adc1_in1_data);


        if adc1_in1_data > 3500 {

            motor1.increment_and_set(1);

            iprintln!(&mut itm, "Increasing duty currentlty {}", motor1.get_duty());
        }

        else if adc1_in1_data < 500 {

            motor1.decrement_and_set(1);

            iprintln!(&mut itm, "Decreasing  duty currentlty {}", motor1.get_duty());
        }


    }
}


struct Motor1WithLed1 {
    pwm_channel: pwm::PwmChannel<pwm::TIM3_CH2, pwm::WithPins>,
    led: gpio::gpioe::PE9<gpio::Output<gpio::PushPull>>,
    led_on: bool,
    duty: u16,
    min: u16,
    max: u16
}

impl Motor1WithLed1 {


    pub fn new(mut pwm_channel: pwm::PwmChannel<pwm::TIM3_CH2, pwm::WithPins>, led: gpio::gpioe::PE9<gpio::Output<gpio::PushPull>>) -> Self {

        let min = pwm_channel.get_max_duty() / 20;
        let max = pwm_channel.get_max_duty() / 10;

        pwm_channel.set_duty(min);
        pwm_channel.enable();

        Motor1WithLed1 {
            pwm_channel,
            led,
            led_on: false,
            duty: min,
            min,
            max
        }
    }

    pub fn get_duty(&self) -> u16 {
        self.duty
    }

    pub fn increment_and_set(&mut self, inc: u16) {
        self.duty = core::cmp::min(self.max, self.duty + inc);
        self.pwm_channel.set_duty(self.duty);

        if !self.led_on {
            self.led_on = true;
            self.led.set_high().unwrap();
        }

    }

    pub fn decrement_and_set(&mut self, dec: u16) {

        self.duty = core::cmp::max(self.min, self.duty - dec);
        self.pwm_channel.set_duty(self.duty);

        if self.led_on && self.duty == self.min {
            self.led_on = false;
            self.led.set_low().unwrap();
        }

    }

    pub fn set_min(&mut self) {

        // set pwm to min
        self.duty = self.min;
        self.pwm_channel.set_duty(self.duty);

        // Set led to off
        self.led.set_low().unwrap();
        self.led_on = false;

    }
}
