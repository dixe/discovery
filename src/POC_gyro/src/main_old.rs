e#![deny(unsafe_code)]
    #![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};
use aux14::i2c1::RegisterBlock;

const MAGNETOMETER: u16 = 0b0011_1100;

// Addresses of the magnetometer's registers.
const WHO_AM_I_M: u8 = 0x4F;


const OUTX_L_REG_M: u8 = 0x068; // In book it is 'const OUT_X_H_M: u8 = 0x03;' but with new LSM303AGR we need to use this



#[entry]
fn main() -> ! {

    // From thread here: https://github.com/rust-embedded/discovery/issues/274

    let (i2c1, mut delay, mut itm) = aux14::init();


    //let cfg_reg_a_m_byte = set_mode_continuous__LSM303AGR(i2c1);


    let  who_am_i_byte = who_am_i(i2c1);

    // Expected output:  0b01000000(64)
    let expected_output = 64;

    iprintln!(&mut itm.stim[0], "0x{:02X} - 0b{:08b} is expected {}", WHO_AM_I_M, who_am_i_byte, who_am_i_byte == expected_output);

    loop {

        // ask for an array of 6 register values starting at OUTX_L_REG_M (0x68)
        {
            // Broadcast START
            // Broadcast the MAGNETOMETER address with the R/W bit set to Write
            i2c1.cr2.write(|w| {
                w.start().set_bit();
                w.sadd().bits(MAGNETOMETER);
                w.rd_wrn().clear_bit();
                w.nbytes().bits(1);
                w.autoend().clear_bit()
            });

            // Wait until we can send more data
            while i2c1.isr.read().txis().bit_is_clear() {}

            // Send the address of the register that we want to read: WHO_AM_I_M
            i2c1.txdr.write(|w| w.txdata().bits(OUTX_L_REG_M));

            // Wait until the previous byte has been transmitted
            while i2c1.isr.read().tc().bit_is_clear() {}
        }

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

        iprintln!(&mut itm.stim[0], "{:?}", buffer);

        delay.delay_ms(1_000_u16);
    }


}


fn set_mode_continuous__LSM303AGR(i2c1: &RegisterBlock) -> (u8) {
    {
        // Broadcast START
        // Broadcast the MAGNETOMETER address with the R/W bit set to Write
        i2c1.cr2.write(|w| {
            w.start().set_bit();
            w.sadd().bits(MAGNETOMETER);
            w.rd_wrn().clear_bit();
            w.nbytes().bits(2);
            w.autoend().clear_bit()
        });

        // Wait until we can send more data
        while i2c1.isr.read().txis().bit_is_clear() {}


        // Send the address of the register that we want to read: WHO_AM_I_M
        i2c1.txdr.write(|w| w.txdata().bits(CFG_REG_A_M));
        i2c1.txdr.write(|w| w.txdata().bits(0x0));

        // Wait until the previous byte has been transmitted
        while i2c1.isr.read().tc().bit_is_clear() {}
    }

    let res_byte = {
        // Broadcast START
        // Broadcast the MAGNETOMETER address with the R/W bit set to Write
        i2c1.cr2.write(|w| {
            w.start().set_bit();
            w.sadd().bits(MAGNETOMETER);
            w.rd_wrn().clear_bit();
            w.nbytes().bits(1);
            w.autoend().set_bit()
        });

        // wait until we can read from register
        while i2c1.isr.read().rxne().bit_is_clear() {}

        // Broadcast STOP (automatic because of `AUTOEND = 1`)
        i2c1.rxdr.read().rxdata().bits()
    };

    res_byte
}


fn who_am_i(i2c1: &RegisterBlock) -> u8 {
    // Stage 1: Send the address of the register we want to read to the
    // magnetometer
    {
        // Broadcast START
        // Broadcast the MAGNETOMETER address with the R/W bit set to Write
        i2c1.cr2.write(|w| {
            w.start().set_bit();
            w.sadd().bits(MAGNETOMETER);
            w.rd_wrn().clear_bit();
            w.nbytes().bits(1);
            w.autoend().clear_bit()
        });

        // Wait until we can send more data
        while i2c1.isr.read().txis().bit_is_clear() {}

        // Send the address of the register that we want to read: WHO_AM_I_M
        i2c1.txdr.write(|w| w.txdata().bits(WHO_AM_I_M));

        // Wait until the previous byte has been transmitted
        while i2c1.isr.read().tc().bit_is_clear() {}
    }

    // Stage 2: Receive the contents of the register we asked for
    let byte = {
        // Broadcast RESTART
        // Broadcast the MAGNETOMETER address with the R/W bit set to Read.
        i2c1.cr2.modify(|_, w| {
            w.start().set_bit();
            w.nbytes().bits(1);
            w.rd_wrn().set_bit();
            w.autoend().set_bit()
        });

        // Wait until we have received the contents of the register
        while i2c1.isr.read().rxne().bit_is_clear() {}

        // Broadcast STOP (automatic because of `AUTOEND = 1`)

        i2c1.rxdr.read().rxdata().bits()
    };


    byte
}
/*#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux14::{entry, iprint, iprintln, prelude::*};
/
/ Slave address
const MAGNETOMETER: u16 = 0b0011_1100;

// Addresses of the magnetometer's registers
const OUT_X_H_M: u8 = 0x03;
const IRA_REG_M: u8 = 0x0A;
const WHO_AM_I_M: u8 = 0x4F;

#[entry]
fn main() -> ! {
let (i2c1, _delay, mut itm) = aux14::init();

// Stage 1: Send the address of the register we want to read to the
// magnetometer
{
// Broadcast START
// Broadcast the MGNOMETER address with the R/W/ bit set to write
i2c1.cr2.write(|w| {
w.start().set_bit();
w.sadd().bits(MAGNETOMETER);
w.rd_wrn().clear_bit();
w.nbytes().bits(1);
w.autoend().clear_bit()
        });

        // wait until we can send more data
        while i2c1.isr.read().txis().bit_is_clear() {}
        i2c1.txdr.write(|w| w.txdata().bits(IRA_REG_M));
    }

    // Stage 2: Receive the contents of the register we asked for
    let byte = {

        // Broadcast RESTART
        // Broadcast the MAGNETOMETER address with the R/W bit set to Read
        i2c1.cr2.modify(|_, w| {
            w.start().set_bit();
            w.nbytes().bits(1);
            w.rd_wrn().set_bit();
            w.autoend().set_bit()
        });

        // wait until we have recieved the contents of the register

        while i2c1.isr.read().rxne().bit_is_clear() {}


        // TODO Broadcast STOP (automatic because of `AUTOEND = 1`)
        i2c1.rxdr.read().rxdata().bits()
    };

    // Expected output: 0x0A - 0b01001000(64)
    iprintln!(&mut itm.stim[0], "0x{:02X} - 0b{:08b}", IRA_REG_M, byte);

    loop {}
}
*/
