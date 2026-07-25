use usbd_hid::{hid_class::HIDClass};
use usbd_hid::descriptor::generator_prelude::*;

use usb_device::bus::{UsbBus, UsbBusAllocator};
use usb_device::prelude::UsbDevice;
use usb_device::prelude::UsbDeviceBuilder;
use usb_device::prelude::UsbVidPid;
use usb_device::device::UsbRev;
use usb_device::prelude::StringDescriptors;
use usb_device::LangID;

pub type MyUsb<'a, B> = (UsbDevice<'a, B> , HIDClass<'a, B> );

pub fn init_usb<'a, B>(
    usb_bus: &'a UsbBusAllocator<B>
) -> MyUsb<'a, B>
where
    B: UsbBus
{
    #[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = VENDOR_DEFINED_START, usage = 0x01) = {
        input_buffer=input;
        output_buffer=output;
    }
    )]
    struct CustomBidirectionalReport {
	input_buffer: [u8; 32],
	output_buffer: [u8; 32],
    }
    let hid = HIDClass::new(usb_bus, CustomBidirectionalReport::desc(), 10); // 10ms Polling-Intervall
    let usb_dev = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x1209, 0x0001))
        .strings(&[
           StringDescriptors::new(LangID::EN)
                .manufacturer("DM1MF")
                .product("RigToUSB")
                .serial_number("TEST")
        ])
	.expect("Cannot set USB String Descriptors")
	.usb_rev(UsbRev::Usb200)
	.device_class(0x00)
	.build();
  (usb_dev, hid)
}
