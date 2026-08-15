use usb_device::class_prelude::UsbBus;
use usb_device::prelude::*;
use usbd_hid::descriptor::gen_hid_descriptor;
use usbd_hid::descriptor::SerializedDescriptor;
use usbd_hid::descriptor::AsInputReport;

use crate::usb::{MyUsb};
use crate::tasks::Tasks;

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = 0xFF00, usage = 0x01) = {
            data = input
    }
)]
pub struct RawReport {
    pub data: [u8; 32],
}

pub fn hid_push<'a, B: UsbBus>
    (my_usb : &mut MyUsb<'a , B>,
    mut tasks : Tasks,
    ) -> Tasks
{
    if let Some(chr) = tasks.rx_cat_char {
	let mut packet = RawReport {
	    data: [0u8; 32],
	};
	packet.data[0] = chr;
	let (_usb_dev, hid) = my_usb;
	match hid.push_input(&packet) {
	    Ok(_bytes_sent) => {
                defmt::info!("Push hid: {:a}", chr );
	    }
	    Err(UsbError::WouldBlock) => {
                defmt::info!("WouldBlock");
	    }
	    Err(_) => {
                defmt::info!("ERR_");
            }
	}
    }
    tasks.rx_cat_char = None;
    tasks
}
