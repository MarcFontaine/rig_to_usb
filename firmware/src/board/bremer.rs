use core::mem;
use crate::hal::{*};

use gpio::PinState;
use gpio::PinState::*;

use crate::uart_tx;
use crate::uart_iterator;

pub type LedPin = Pin<'C',13, Output<PushPull>>;
pub type Hochschalten = Pin<'C',14, Output<PushPull>>;
pub type OnOff = Pin<'C',15, Output<PushPull>>;

pub struct Board {
    pub led: LedPin,
    pub hochschalten: Hochschalten,
    pub on_off: OnOff,
    pub cat_tx: uart_tx::TxState,
    pub cat_rx: uart_iterator::PollingReceiver,
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

    #[inline(always)]
    pub fn send_cat (&mut self, bytes: &[u8]) {
	self.cat_tx = uart_tx::send_uart(
	    mem::replace(&mut self.cat_tx, uart_tx::Locked)
		, bytes);
    }
}
