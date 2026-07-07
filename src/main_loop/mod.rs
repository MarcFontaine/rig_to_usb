use usb_device::prelude::UsbDevice;
use usb_device::class_prelude::UsbBus;
use usbd_hid::hid_class::HIDClass;

use crate::cmd::{decode_and_run};

pub fn main_loop<B: UsbBus>
    (usb_dev: &mut UsbDevice<'_, B>,
     hid: &mut HIDClass<'_, B>,
    ) -> !
{
    let mut rx_packet = [0u8; 64];
    defmt::info!("Starting Loop");
    loop {	
        if usb_dev.poll(&mut [hid]) {
            match hid.pull_raw_output(&mut rx_packet) {
                Ok(bytes_read) if bytes_read > 0 => {
		    decode_and_run(&rx_packet[..bytes_read]);
                }
                _ => {}
            }
        }
    }
}
