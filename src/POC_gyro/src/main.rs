#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14_accel::{entry, iprint, iprintln, prelude::*};


#[entry]
fn main() -> ! {
    let (i2c1,  mut lsm303agr, mut delay, mut itm) = aux14_accel::init();

    loop {

        if lsm303agr.accel_status().unwrap().xyz_new_data {
            let data = lsm303agr.accel_data().unwrap();

            iprintln!(&mut itm.stim[0], "Acceleration: x {} y {} z {}", data.x, data.y, data.z);
        }
        else {
            iprintln!(&mut itm.stim[0], "No new data");
        }


    }
}
