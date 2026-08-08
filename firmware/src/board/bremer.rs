use core::mem;

use crate::hal::{*};
use crate::hal;
use stm32f1xx_hal::dma::WriteDma;

use gpio::PinState;
use gpio::PinState::*;

pub type LedPin = Pin<'C',13, Output<PushPull>>;
pub type Hochschalten = Pin<'C',14, Output<PushPull>>;
pub type OnOff = Pin<'C',15, Output<PushPull>>;

pub type UartTxTransfer = hal::dma::Transfer<
    hal::dma::R,
    &'static mut [u8],
    serial::TxDma1
>;

pub enum TxState {
    Ready(serial::TxDma1),
    Busy(UartTxTransfer),
    Locked,
}

pub struct Board {
    pub led: LedPin,
    pub hochschalten: Hochschalten,
    pub on_off: OnOff,
    pub cat_tx: TxState
}

const CAT_TX_BUFFER_LEN : usize = 50;
static mut CAT_TX_BUFFER: [u8; CAT_TX_BUFFER_LEN] = [0; CAT_TX_BUFFER_LEN];

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
        let mut tx_state = mem::replace(&mut self.cat_tx, TxState::Locked);
	if let TxState::Busy(ref tx) = tx_state {
	    if tx.is_done() {
		if let TxState::Busy(tx) = tx_state {
		    let (_recovered_buf, tx_new) = tx.wait();
		    tx_state = TxState::Ready(tx_new);
		}
	    }
	}
        match tx_state {
	    TxState::Ready(tx_dma) => {
		let static_buf: &'static mut [u8] = unsafe {
                    let ptr = core::ptr::addr_of_mut!(CAT_TX_BUFFER);
                    core::slice::from_raw_parts_mut(ptr as *mut u8, CAT_TX_BUFFER_LEN)
		};
		let len = bytes.len().min(CAT_TX_BUFFER_LEN);
		static_buf[..len].copy_from_slice(&bytes[..len]);
		let to_send = &mut static_buf[..len];
		let transfer = tx_dma.write(to_send);
		self.cat_tx = TxState::Busy(transfer);
	    }
	    TxState::Busy(_) => {
		self.cat_tx = tx_state;
	    }
	    TxState::Locked => {
	    }
	}
    }
}
