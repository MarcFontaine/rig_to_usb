use minicbor;

use ringbuffer::RingBuffer;
use usb_device::class_prelude::UsbBus;
use usb_device::prelude::*;

use crate::config::XK852_MAX_LINE_LEN;
use crate::usb::{MyUsb};
use crate::xk852line;
use rig_to_usb_logic::hid_report::HIDReport;

pub fn hid_push<'a, B: UsbBus>
    (my_usb : &mut MyUsb<'a , B>,
    ls: &mut xk852line::LineState)
{
    if let xk852line::LineState::ParseResult{msg, prefix} = ls {
        let mut rx = [0u8; XK852_MAX_LINE_LEN];
	for (slot, &val) in rx.iter_mut().zip(msg.iter().take(XK852_MAX_LINE_LEN)) {
            *slot = val;
	}
        let mut skipped = [0u8; XK852_MAX_LINE_LEN];
	for (slot, &val) in skipped.iter_mut().zip(prefix.iter().take(XK852_MAX_LINE_LEN)) {
            *slot = val;
	}
        let report = HIDReport {
	    cat_rx_msg: Some(&rx[..msg.len()]),
	    cat_rx_skipped: Some(&skipped[..prefix.len()]),
	};
	let mut packet = [0u8; 64];
        let mut cursor1 = minicbor::encode::write::Cursor::new(&mut packet[..]);
        let Ok(_encoded_s) = minicbor::encode(&report, &mut cursor1) else {
            // TODO: HIDReport may not fit in 64 Bytes !!
            defmt::info!("minicbor encode error");
            *ls = xk852line::init_line_state();
            return;
        };
	let (_usb_dev, hid) = my_usb;
	match hid.push_raw_input(&packet) {
	    Ok(_bytes_sent) => {
                defmt::info!("Push hid: {:?}", packet );
                *ls = xk852line::init_line_state();
		return
	    }
	    Err(UsbError::WouldBlock) => {
                defmt::info!("WouldBlock");
	    }
	    Err(_) => {
                defmt::info!("ERR_");
            }
	}
    }
}
