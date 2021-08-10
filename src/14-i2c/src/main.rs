










// USE POC_ACCEL NOT THIS, THIS DOES NOT WORK, IT MIGHT COMPILE
// BUT OUTPUT IS NOT CORRECT, SETUP NOT WORKING











#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};


// Slave address
const MAGNETOMETER: u16 = 0b0011_1100;

// Addresses of the magnetometer's registers
const OUT_X_H_M: u8 = 0x068;// In book it is 'const OUT_X_H_M: u8 = 0x03;' but with new LSM303AGR we need to use this 0x68

const CTRL_REG1_A: u8 = 0x020;


const ACC_STARTUP: u8 = 0x057;

#[entry]
fn main() -> ! {
    let (i2c1, mut delay, mut itm, lsm) = aux14::init();

    // Broadcast START
    // Broadcast the MAGNETOMETER address with the R/W bit set to Write (w.rd_wrn.clear_bit())
    i2c1.cr2.write(|w| {
        w.start().set_bit();
        w.sadd().bits(MAGNETOMETER);
        w.rd_wrn().clear_bit();
        w.nbytes().bits(2);
        w.autoend().clear_bit()
    });

    // Wait until we can send more data
    while i2c1.isr.read().txis().bit_is_clear() {}
    // send address of the register we want to write
    i2c1.txdr.write(|w| w.txdata().bits(CTRL_REG1_A));

    // send the data
    i2c1.txdr.write(|w| w.txdata().bits(ACC_STARTUP));

    // Wait until the previous byte has been transmitted
    while i2c1.isr.read().tc().bit_is_clear() {}




    loop {
        // Broadcast START
        // Broadcast the MAGNETOMETER address with the R/W bit set to Write (w.rd_wrn.clear_bit())
        i2c1.cr2.write(|w| {
            w.start().set_bit();
            w.sadd().bits(MAGNETOMETER);
            w.rd_wrn().clear_bit();
            w.nbytes().bits(1);
            w.autoend().clear_bit()
        });

        // Wait until we can send more data
        while i2c1.isr.read().txis().bit_is_clear() {}

        // Send the address of the register that we want to read: OUT_X_H_M
        i2c1.txdr.write(|w| w.txdata().bits(OUT_X_H_M));

        // Wait until the previous byte has been transmitted
        while i2c1.isr.read().tc().bit_is_clear() {}

        // Broadcast RESTART
        // Broadcast the MAGNETOMETER address with the R/W bit set to Read (w.rd_wrn.set_bit())

        // Set 6 bytes continuesly to be read, starting from register OUT_X_H_M, that we set at the start
        i2c1.cr2.modify(|_, w| {
            w.start().set_bit();
            w.nbytes().bits(6);
            w.rd_wrn().set_bit();
            w.autoend().set_bit()
        });

        let mut buffer = [0u8; 6];
        for byte in &mut buffer {
            // Wait until we have received something
            while i2c1.isr.read().rxne().bit_is_clear() {}

            *byte = i2c1.rxdr.read().rxdata().bits();
        }
        // Broadcast STOP (automatic because of `AUTOEND = 1`)



        let x_l = u16::from(buffer[0]);
        let x_h = u16::from(buffer[1]);
        let y_l = u16::from(buffer[2]);
        let y_h = u16::from(buffer[3]);
        let z_l = u16::from(buffer[4]);
        let z_h = u16::from(buffer[5]);

        let x = ((x_l + (x_h << 8)) as i16);
        let y = (y_l + (y_h << 8)) as i16;
        let z = (z_l + (z_h << 8)) as i16;

        let accel_status = lsm.accel_status();

        iprintln!(&mut itm.stim[0], "status = {:?}x={:?} y={:?} z={:?}",accel_status, x,y,z);

        delay.delay_ms(2_00_u16);
    }
}
