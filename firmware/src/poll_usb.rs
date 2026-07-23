use usb_device::class_prelude::UsbBus;

use crate::init::{BoardPeripherals};
use crate::cmd::{decode_and_run};
use crate::usb::{MyUsb};

pub fn poll_usb<'a, B: UsbBus>
    (boardboard_peripherals: &mut BoardPeripherals
    ,my_usb : &mut MyUsb<'a , B>
    ) -> ()
{
    let mut rx_packet = [0u8; 64];
    let (usb_dev, hid) = my_usb;
    if usb_dev.poll(&mut [hid]) {
	match hid.pull_raw_output(&mut rx_packet) {
	    Ok(bytes_read) if bytes_read > 0 => {
		decode_and_run(boardboard_peripherals, &rx_packet[..bytes_read]);
	    }
	    _ => {}
	}
    }
}
