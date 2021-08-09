#![no_std]
#![no_main]

//! Example usage for ADC on STM32F303

use panic_itm as _; // panic handler

use cortex_m::{asm, iprintln};
use cortex_m_rt::entry;
use stm32f3xx_hal::{adc, pac, prelude::*};

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
    let mut adc1_in1_pin = gpioa.pa1.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);

    loop {

        let adc1_in1_data: u16 = adc1.read(&mut adc1_in1_pin).expect("Error reading from adc1.");

        iprintln!(&mut itm, "PA1 reads {}", adc1_in1_data);

    }
}
