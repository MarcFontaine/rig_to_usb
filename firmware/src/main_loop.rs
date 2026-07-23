use usb_device::class_prelude::UsbBus;

use crate::init::{BoardPeripherals};
use crate::usb::{MyUsb};
use crate::poll_usb::{poll_usb};
use cortex_m::peripheral::DWT;

pub fn main_loop<'a, B: UsbBus>
    (board_peripherals: &mut BoardPeripherals
    ,my_usb : &mut MyUsb<'a , B>
    ) -> !
{
    let mut ping = DWT::cycle_count().wrapping_add(10*72000000);
    defmt::info!("Starting Loop");
    loop {
	poll_usb(board_peripherals, my_usb);
	if ping.wrapping_sub(DWT::cycle_count()) > (1<<31) {
            defmt::info!("Ping");
	    ping = DWT::cycle_count().wrapping_add(10*72000000);
	}
    }
}
