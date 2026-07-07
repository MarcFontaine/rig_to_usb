use usb_device::prelude::UsbDevice;
use usb_device::class_prelude::UsbBus;
use usbd_hid::hid_class::HIDClass;

use crate::cmd::{MessageIn};

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
                    let mut decoder = minicbor::Decoder::new(&rx_packet[..bytes_read]);
                    if let Ok(msg) = decoder.decode::<MessageIn>() {
		        defmt::info!("cbor decode message: {}", msg);
                    } else
		    {
                        defmt::info!("cbor decode error");
		        defmt::info!("cbor decode daten: {=[u8]:x}", rx_packet);
		        defmt::info!("cbor decode daten: {}", rx_packet);
		    }
                    
                }
                _ => {}
            }
        }
    }
}
