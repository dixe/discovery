#![no_main]
#![no_std]

use panic_itm as _; // panic handler

use core::ptr;

#[allow(unused_imports)]
use aux7::{entry, iprintln, ITM};

#[entry]
fn main() -> ! {
    let gpioe = aux7::init().1;

    gpioe.bsrr.write(|w| w.bs9().set_bit());

    gpioe.bsrr.write(|w| w.bs11().set_bit());

    gpioe.bsrr.write(|w| w.br9().set_bit());

    gpioe.bsrr.write(|w| w.br11().set_bit());


    loop {}
}


fn iprint_odr(itm: &mut ITM) {

    const GPIOE_ODR: u32 = 0x4800_1014;

    unsafe {
        iprintln!(
            &mut itm.stim[0],
            "ODR = 0x{:04x}",
            ptr::read_volatile(GPIOE_ODR as *const u16));
    }

}
