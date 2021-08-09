#![no_std]
#![no_main]



//! Example usage for ADC on STM32F303

use panic_itm as _; // panic handler

use cortex_m::{asm, iprintln};
use cortex_m_rt::entry;
use stm32f3xx_hal::{
    adc,
    prelude::*,
    pac::{self, RCC},
    delay::Delay,
    flash::FlashExt,
    gpio::{self, GpioExt},
    rcc::{self, RccExt},
    pwm::{self, tim16, tim2, tim3, tim8},
};

use stm32f3xx_hal::hal::{PwmPin, blocking::delay::DelayMs,};


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



// Based on https://github.com/stm32-rs/stm32f3xx-hal/blob/master/examples/pwm.rs
#[entry]
fn main() -> ! {

    let mut dp = pac::Peripherals::take().unwrap();

    let mut cp = pac::CorePeripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    let mut itm = &mut cp.ITM.stim[0];

    let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);
    let mut delay = Delay::new(cp.SYST, clocks);


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


    let mut led3 = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    let pa4 = gpioa
        .pa4
        .into_af2_push_pull(&mut gpioa.moder, &mut gpioa.otyper, &mut gpioa.afrl);


    let pa0 = gpioa
        .pa0
        .into_input(&mut gpioa.moder);


    // each channel can have different duty cycle
    let mut tim3_ch2 = tim3_ch2_nopin.output_to_pa4(pa4);


    // min should be 1 ms
    // max should be 2 ms
    // freq is 50 hz so that 20 ms pr cycle

    let min = tim3_ch2.get_max_duty() / 20;
    let max = tim3_ch2.get_max_duty() / 10;

    tim3_ch2.set_duty(min);
    tim3_ch2.enable();


    let duty = u16::from(tim3_ch2.get_duty());

    iprintln!(&mut itm, "Duty reads {}", duty);

    let init = min;
    tim3_ch2.set_duty(init);

    iprintln!(&mut itm, "Init pwm duty set to {}", init);

    let mut pressed = false;

    let mut pwm_state_high = init == max;

    loop {
        let duty = tim3_ch2.get_duty();

        match toogle_pa0(pressed, &pa0) {
            ButtonAction::Pressed => {

                pressed = true;
                pwm_state_high = !pwm_state_high;

                let new_duty = if pwm_state_high
                {
                    max
                } else {
                    min
                };

                if max == new_duty {
                    led3.set_high();
                }

                if min == new_duty {
                    led3.set_low();
                }
                tim3_ch2.set_duty(new_duty);
                iprintln!(&mut itm, "user btn press, pwm duty set to {}", new_duty);

            },
            ButtonAction::Released => {
                //iprintln!(&mut itm, "user btn release");
                pressed = false;
            },
            ButtonAction::NoAction => {}
        }

        let high = pa0.is_high().unwrap();


        //iprintln!(&mut itm, "Duty reads {} - max={} pa0 high = {}", duty, max, high);
        //        tim3_ch2.set_duty(max/ 2);

        //delay.delay_ms(1_000_u16);

    }
}

enum ButtonAction {
    NoAction,
    Pressed,
    Released
}


fn toogle_pa0(pressed: bool, pa0: &gpio::gpioa::PA0<gpio::Input>) -> ButtonAction {
    let high = pa0.is_high().unwrap();

    if !pressed && high {
        // new press react
        return ButtonAction::Pressed
    }

    if pressed && !high {
        // released, reset pressed
        return ButtonAction::Released
    }

    // no pressed and not high, don't care
    // Or pressed and still high
    ButtonAction::NoAction

}
