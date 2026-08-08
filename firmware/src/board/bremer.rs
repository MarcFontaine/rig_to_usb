use crate::hal::{*};

use gpio::PinState;
use gpio::PinState::*;

pub type LedPin = Pin<'C',13, Output<PushPull>>;
pub type Hochschalten = Pin<'C',14, Output<PushPull>>;
pub type OnOff = Pin<'C',15, Output<PushPull>>;

pub struct Board {
    pub led: LedPin,
    pub hochschalten: Hochschalten,
    pub on_off: OnOff,
}

impl Board {
    #[inline(always)]
    pub fn led_on (&mut self) {
        self.led.set_state(Low);
    }
    #[inline(always)]
    pub fn led_off (&mut self) {
        self.led.set_state(High);
    }

    #[inline(always)]
    pub fn led_set (&mut self, value: bool) {
        self.led.set_state(PinState::from(!value));
    }

    #[inline(always)]
    pub fn radio_on (&mut self) {
        self.on_off.set_state(Low);
    }
    #[inline(always)]
    pub fn radio_off (&mut self) {
        self.on_off.set_state(High);
    }

    #[inline(always)]
    pub fn tx_on (&mut self) {
        self.hochschalten.set_state(Low);
    }
    #[inline(always)]
    pub fn tx_off (&mut self) {
        self.hochschalten.set_state(High);
    }
}

