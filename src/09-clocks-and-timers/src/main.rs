#![no_main]
#![no_std]

use cortex_m::asm::{bkpt, nop};
use aux9::{entry, switch_hal::OutputSwitch, tim6};

use stm32f3xx_hal::interrupt;

/*
#[inline(never)]
fn delay(tim7: &tim7::RegisterBlock, ms: u16) {
// TODO implement this
// set ticks to count to, timer is 1 khz so ms == ticks
tim7.arr.write(|w| w.arr().bits(ms));

// enable timer to start counting
tim7.cr1.modify(|_, w| w.cen().set_bit());

// wait for update event
while !tim7.sr.read().uif().bit_is_set() {}

// clear update flag to not exit immediatly next delay
tim7.sr.modify(|_,w| w.uif().clear_bit());
}
*/


#[interrupt]
fn TIM7() {
    bkpt();
}


#[entry]
fn main() -> ! {
    let (leds, rcc, tim7) = aux9::init();
    let mut leds = leds.into_array();

    // TODO initialize TIM7

    rcc.apb1enr.modify(|_,w| w.tim7en().set_bit());

    tim7.cr1.write(|w| w.urs().set_bit());


    let psc = 7999;

    tim7.psc.write(|w| w.psc().bits(psc));

    // set interupt
    tim7.dier.write(|w| w.uie().set_bit());

    // set ticks to count to, timer is 1 khz so ms == ticks
    tim7.arr.write(|w| w.arr().bits(1000));

    // enable timer to start counting
    tim7.cr1.modify(|_, w| w.cen().set_bit());

    let ms = 50;
    let mut cur = 0;
    loop {
        if tim7.sr.read().uif().bit_is_set() {
            leds[cur].on().unwrap();
            cur +=1;
            tim7.sr.modify(|_,w| w.uif().clear_bit());
        }
    }
}
