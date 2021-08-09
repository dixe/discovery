#![deny(unsafe_code)]
#![no_main]
#![no_std]

use volatile::Volatile;
use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {

    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let cycle_time = 50_u16;

    let mut next = 0;

    let mut off = 0;
    loop {


        let off = (8 + next - 2) % 8;
        leds[next].on().ok();
        leds[off].off().ok();

        delay.delay_ms(cycle_time);

        next = (next + 1) % 8;


    }
}
