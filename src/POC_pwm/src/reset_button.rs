use stm32f3xx_hal::{
    prelude::*,
    gpio::{self},
};


enum ButtonAction {
    NoAction,
    Pressed,
    Released
}


pub struct ResetButton {
    pa0: gpio::gpioa::PA0<gpio::Input>,
    pressed: bool
}

impl ResetButton {

    pub fn new(pa0: gpio::gpioa::PA0<gpio::Input>) -> Self {
        ResetButton {
            pa0,
            pressed: false
        }
    }


    pub fn check_reset_press<F>(&mut self, on_press: F) where F: FnOnce(){

        match self.get_button_state() {
            ButtonAction::Pressed => {
                self.pressed = true;
                on_press();
            },
            ButtonAction::Released => {
                //iprintln!(&mut itm, "user btn release");
                self.pressed = false;
            },
            ButtonAction::NoAction => {}
        }
    }

    fn get_button_state(&mut self) -> ButtonAction {
        let high = self.pa0.is_high().unwrap();

        if !self.pressed && high {
            // new press react
            return ButtonAction::Pressed
        }

        if self.pressed && !high {
            // released, reset pressed
            return ButtonAction::Released
        }

        // no pressed and not high, don't care
        // Or pressed and still high
        ButtonAction::NoAction
    }

}
