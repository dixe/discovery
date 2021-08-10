use stm32f3xx_hal::{
    prelude::*,
    gpio::{self},
    pwm::{self},
};


pub struct Motor1WithLed1 {
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
