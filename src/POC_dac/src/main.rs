#![no_std]
#![no_main]

//! Example usage for ADC on STM32F303

use panic_itm as _; // panic handler

use cortex_m::{asm, iprintln};
use cortex_m_rt::entry;
use stm32f3xx_hal::{adc, prelude::*, pac::{self, RCC}};

// See https://github.com/arocketman/stm32f3-discovery-projects/blob/master/12-DAC-to-generate-a-sinusoid/main.c for C
// and https://gist.github.com/strom-und-spiele/8155f19258adfd66214d2d0aac956a43 for rust
// for issue https://github.com/stm32-rs/stm32f3xx-hal/issues/51



#[entry]
/// Main Thread
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    let mut rcc = peripherals.RCC.constrain();
    let mut gpio_a = peripherals.GPIOA.split(&mut rcc.ahb);

    let adc1 = peripherals.ADC1;

    // ADC12 "split"
    let adc12 = peripherals.ADC1_2;

    // DMA1 "split"
    let dma1 = peripherals.DMA1;

    // set up pin pa0, pa1 as analog pin
    let _adc1_in1_pin = gpio_a.pa0.into_analog(&mut gpio_a.moder, &mut gpio_a.pupdr);
    let _adc1_in2_pin = gpio_a.pa1.into_analog(&mut gpio_a.moder, &mut gpio_a.pupdr);
    let _dac1_out1_pin = gpio_a.pa4.into_analog(&mut gpio_a.moder, &mut gpio_a.pupdr);
    let _dac1_out2_pin = gpio_a.pa5.into_analog(&mut gpio_a.moder, &mut gpio_a.pupdr);

    // enabling adc, dac and dma clk (there is a crate only API for that)
    let ahbenr = unsafe { &(*RCC::ptr()).ahbenr };
    let apb1enr = unsafe { &(*RCC::ptr()).apb1enr };

    apb1enr.modify(|_, w| w.dac1en().enabled());
    ahbenr.modify(|_, w| w.adc12en().enabled().dma1en().enabled());

    // set adc clk config accordingly with prescale = 4
    unsafe {
        adc12.ccr.modify(|_, w| w.ckmode().bits(0b11));
    }

    // init target for dma
    let adc_values: [u16; 2] = [0x07ff; 2];

    // Set up DMA channels
    //  The following sequence should be followed to configure a DMA channel x (where x is the channel number).
    //  1.  Set the peripheral register address in the DMA_CPARx register. The data will be moved
    //      from/ to this address to/ from the memory after the peripheral event.
    unsafe {
        dma1.ch1
            .par
            .write(|w| w.pa().bits(&adc1 as *const _ as u32 + 0x40));
        //  2.  Set the memory address in the DMA_CMARx register. The data will be written to or
        //      read from this memory after the peripheral event.

        dma1.ch1
            .mar
            .write(|w| w.ma().bits(&adc_values as *const u16 as u32));
        //  3.  Configure the total number of data to be transferred in the DMA_CNDTRx register.  After
        //      each peripheral event, this value will be decremented.
    }
    dma1.ch1.ndtr.write(|w| w.ndt().bits(2));

    //  4.  Configure the channel priority using the PL[1:0] bits in the DMA_CCRx register
    dma1.ch1.cr.modify(|_, w| w.pl().medium());

    //  5.  Configure data transfer direction, circular mode, peripheral & memory incremented mode,
    //      peripheral & memory data size, and interrupt after half and/or full transfer in the
    //      DMA_CCRx register
    #[rustfmt::skip]
    dma1.ch1.cr.modify(|_, w| { w
                                .dir().from_peripheral()
                                .circ().enabled()
                                .pinc().disabled()
                                .minc().enabled()
                                .psize().bits16()
                                .msize().bits16()
                                .teie().disabled()
                                .tcie().disabled()
    });

    //  6.  Activate the channel by setting the ENABLE bit in the DMA_CCRx register.  As soon as
    //      the channel is enabled, it can serve any DMA request from the peripheral connected on
    //      the channel.
    dma1.ch1.cr.modify(|_, w| w.en().enabled());

    // /////////////  set up dac
    let dac = pac::DAC1::ptr();
    let dac_cr = unsafe { &(*dac).cr };
    let dac_swtrigr = unsafe { &(*dac).swtrigr };
    let dac_dhr12rd = unsafe { &(*dac).dhr12rd };

    // enable both dac channels and triggers
    #[rustfmt::skip]
    dac_cr.modify(|_, w| { w
                           .ten1().enabled()
                           .ten2().enabled()
                           .tsel1().software()
                           .tsel2().software()
    });
    dac_cr.modify(|_, w| w.en1().enabled().en2().enabled());

    loop {
        #[rustfmt::skip]
        dac_dhr12rd.write(|w| unsafe { w
                                       .dacc1dhr().bits(adc_values[0])
                                       .dacc2dhr().bits(adc_values[1])
        });

        #[rustfmt::skip]
        dac_swtrigr.write(|w| w.
                          swtrig1().enabled()
                          .swtrig2().enabled()
        );
    }
}

/*
#[entry]
fn main() -> ! {

let mut dp = pac::Peripherals::take().unwrap();

let mut cp = pac::CorePeripherals::take().unwrap();
let mut itm = &mut cp.ITM.stim[0];

let mut rcc = dp.RCC.constrain();
let mut gpio_a = dp.GPIOA.split(&mut rcc.ahb);

let _dac1_out1_pin = gpio_a.pa4.into_analog(&mut gpio_a.moder, &mut gpio_a.pupdr);
let _dac1_out2_pin = gpio_a.pa5.into_analog(&mut gpio_a.moder, &mut gpio_a.pupdr);


// enabling dac
let apb1enr = unsafe { &(*RCC::ptr()).apb1enr };
apb1enr.modify(|_, w| w.dac1en().enabled());


let dac = pac::DAC1::ptr();
let dac_cr = unsafe { &(*dac).cr };
let dac_swtrigr = unsafe { &(*dac).swtrigr };
let dac_dhr12rd = unsafe { &(*dac).dhr12rd };

// enable both channels and triggers
#[rustfmt::skip]
dac_cr.modify(|_,w| {
w
.ten1().enabled()
.ten2().enabled()
.tsel1().software()
.tsel2().software()
});


    dac_cr.modify(|_,w| w.en1().enabled().en2().enabled());

    loop {
    dac_dhr12rd.write(|w|
    unsafe {w
    .dacc1dhr().bits(0xFF)
    .dacc2dhr().bits(0x7)
});



    #[rustfmt::skip]
    dac_swtrigr.write(|w| w.
    swtrig1().enabled()
    .swtrig2().enabled()
);

}
}
     */
