use crate::hal::{*};
use gpio::PinState;
use gpio::PinState::*;
pub type LedPin = Pin<'C',13, Output<PushPull>>;

pub struct Led {
    pin : LedPin
}

impl Led {
    #[inline(always)]
    pub fn new(pin: LedPin) -> Self {
        Led { pin }
    }
    #[inline(always)]
    pub fn led_on (&mut self) {
        self.pin.set_state(Low);
    }
    #[inline(always)]
    pub fn led_off (&mut self) {
        self.pin.set_state(High);
    }
    #[inline(always)]
    pub fn set (&mut self, l:bool) {
        self.pin.set_state(PinState::from(l));
    }

}

pub type Hochschalten = Pin<'C',14, Output<PushPull>>;
pub type OnOff = Pin<'C',15, Output<PushPull>>;
