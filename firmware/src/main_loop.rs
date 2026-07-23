use usb_device::prelude::UsbDevice;
use usb_device::class_prelude::UsbBus;
use usbd_hid::hid_class::HIDClass;

use crate::init::{BoardPeripherals};
use crate::cmd::{decode_and_run};
use cortex_m::peripheral::DWT;

pub fn main_loop<B: UsbBus>
    (boardboard_peripherals: &mut BoardPeripherals
    ,usb_dev: &mut UsbDevice<'_, B>
    ,hid: &mut HIDClass<'_, B>
    ) -> !
{
    let mut ping = DWT::cycle_count().wrapping_add(10*72000000);
    let mut rx_packet = [0u8; 64];
    defmt::info!("Starting Loop");
    loop {	
        if usb_dev.poll(&mut [hid]) {
            match hid.pull_raw_output(&mut rx_packet) {
                Ok(bytes_read) if bytes_read > 0 => {
		    decode_and_run(boardboard_peripherals, &rx_packet[..bytes_read]);
                }
                _ => {}
            }
        }
	if ping.wrapping_sub(DWT::cycle_count()) > (1<<31) {
            defmt::info!("Ping");
	    ping = DWT::cycle_count().wrapping_add(10*72000000);
	}
    }
}
