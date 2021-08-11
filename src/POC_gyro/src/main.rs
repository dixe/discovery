#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14_gyro::{entry, iprint, iprintln, prelude::*};


#[entry]
fn main() -> ! {
    let (mut gyro, mut delay, mut itm) = aux14_gyro::init();

    assert_eq!( gyro.who_am_i().unwrap(), 0xD3);
    iprintln!(&mut itm.stim[0], "WHO AM I {}", gyro.who_am_i().unwrap());

    let scale = gyro.scale().unwrap();

    let mut status;
    loop {

        status = gyro.status().unwrap();
        if status.new_data {
            let data = gyro.gyro().unwrap();

            iprintln!(&mut itm.stim[0], "Gyro: x {} y {} z {} Odr = {:?}",
                      scale.degrees(data.x),
                      scale.degrees(data.y),
                      scale.degrees(data.z),
                      gyro.odr().unwrap());
        }
        else {
            iprintln!(&mut itm.stim[0], "NOT READY");
        }

    }
}
