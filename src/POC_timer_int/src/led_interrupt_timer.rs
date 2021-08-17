use stm32f3xx_hal::interrupt;
use stm32f3xx_hal::gpio::gpioe::*;
use stm32f3xx_hal::gpio::*;

use stm32f3xx_hal::{
    prelude::*,
    pac::{TIM7, RCC, NVIC, Interrupt},
};


static mut LED_INTERRUPT_STATE: Option<LedInterruptState> = None;

struct LedInterruptState<'a> {
    leds: [Pin<Gpioe, Ux, Output<PushPull>>; 8],
    cur: usize,
    tim7:  &'a stm32f3xx_hal::pac::tim6::RegisterBlock
}



pub fn setup(mut gpioe: gpioe::Parts )  {

    unsafe {
        // If setup has been called just return
        if let Some(_) = LED_INTERRUPT_STATE {
            return;
        }
    }

    // setup leds

    let leds_s = Leds::new(gpioe.pe8,
                           gpioe.pe9,
                           gpioe.pe10,
                           gpioe.pe11,
                           gpioe.pe12,
                           gpioe.pe13,
                           gpioe.pe14,
                           gpioe.pe15,
                           &mut gpioe.moder,
                           &mut gpioe.otyper);



    let rcc_reg = unsafe { &*RCC::ptr()};
    let tim7 = unsafe { &*TIM7::ptr()};

    // enable timer 7
    rcc_reg.apb1enr.modify(|_,w| w.tim7en().set_bit());

    //
    let psc = 7999;

    tim7.psc.write(|w| w.psc().bits(psc));

    // set interupt
    tim7.dier.write(|w| w.uie().set_bit());

    // set ticks to count to, timer is 1 khz so ms == ticks
    tim7.arr.write(|w| w.arr().bits(1000));

    // enable timer to start counting
    tim7.cr1.modify(|_, w| w.cen().set_bit());


    // Setup the static variable
    unsafe {
        LED_INTERRUPT_STATE = Some(LedInterruptState {
            tim7: tim7,
            leds: leds_s.into_array(),
            cur: 0
        });
    }

    // enable interrupt for TIM7
    unsafe {NVIC::unmask(Interrupt::TIM7) };
}

#[interrupt]
fn TIM7() {

    unsafe {
        if let Some(ref mut lis) = LED_INTERRUPT_STATE {

            // set current low
            lis.leds[lis.cur].set_low().unwrap();
            lis.cur = (lis.cur + 1) % 8;

            // set next high
            lis.leds[lis.cur].set_high().unwrap();

            // reset interrupt status register
            lis.tim7.sr.modify(|_,w| w.uif().clear_bit());
        }
    }
}






struct Leds {
    led0: PE8<Output<PushPull>>,
    led1: PE9<Output<PushPull>>,
    led2: PE10<Output<PushPull>>,
    led3: PE11<Output<PushPull>>,
    led4: PE12<Output<PushPull>>,
    led5: PE13<Output<PushPull>>,
    led6: PE14<Output<PushPull>>,
    led7: PE15<Output<PushPull>>,

}

impl Leds {
    pub fn new(pe8: Pin<Gpioe, U<8_u8>,Input>,
               pe9: Pin<Gpioe, U<9_u8>,Input>,
               pe10: Pin<Gpioe, U<10_u8>,Input>,
               pe11: Pin<Gpioe, U<11_u8>,Input>,
               pe12: Pin<Gpioe, U<12_u8>,Input>,
               pe13: Pin<Gpioe, U<13_u8>,Input>,
               pe14: Pin<Gpioe, U<14_u8>,Input>,
               pe15: Pin<Gpioe, U<15_u8>,Input>,
               moder: &mut gpioe::MODER, otyper:
               &mut gpioe::OTYPER ) -> Self {
        Leds {
            led0 : pe8.into_push_pull_output( moder,  otyper),
            led1 : pe9.into_push_pull_output( moder,  otyper),
            led2 : pe10.into_push_pull_output( moder,  otyper),
            led3 : pe11.into_push_pull_output( moder,  otyper),
            led4 : pe12.into_push_pull_output( moder,  otyper),
            led5 : pe13.into_push_pull_output( moder,  otyper),
            led6 : pe14.into_push_pull_output( moder,  otyper),
            led7 : pe15.into_push_pull_output( moder,  otyper),

        }

    }

    pub fn into_array(self) -> [Pin<Gpioe, Ux, Output<PushPull>>; 8] {

        [self.led0.downgrade(),
         self.led1.downgrade(),
         self.led2.downgrade(),
         self.led3.downgrade(),
         self.led4.downgrade(),
         self.led5.downgrade(),
         self.led6.downgrade(),
         self.led7.downgrade(),

        ]

    }
}
