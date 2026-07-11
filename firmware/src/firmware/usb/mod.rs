use usbd_hid::{hid_class::HIDClass};
use usbd_hid::descriptor::generator_prelude::*;

use stm32g4xx_hal as hal;
use usb_device::prelude::UsbDeviceBuilder;
use usb_device::prelude::UsbVidPid;
use usb_device::prelude::StringDescriptors;
use usb_device::LangID;
use stm32g4xx_hal::gpio::GpioExt;
use usb_device::prelude::UsbDevice;

use crate::init::{BoardPeripherals};
use stm32g4xx_hal::{
    usb::{Peripheral, UsbBus}, 
    pac::USB,
};
use usb_device::bus::UsbBusAllocator;
use static_cell::StaticCell;

static USB_BUS: StaticCell<UsbBusAllocator<UsbBusType>> = StaticCell::new();

pub fn init_usb(usb_peripheral: USB) -> UsbDevice<'static, UsbBusType> { 
    let bus_allocator = UsbBusAllocator::new(usb_peripheral);
    let usb_bus = USB_BUS.init(bus_allocator);

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
    let mut hid = HIDClass::new(&usb_bus, CustomBidirectionalReport::desc(), 10); // 10ms Polling-Intervall
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0x0001))
        .strings(&[
           StringDescriptors::new(LangID::EN)
                .manufacturer("DM1MF")
                .product("RigToUSB")
                .serial_number("TEST")
        ])
	.expect("Cannot set USB String Descriptors") 
	.device_class(0x00)
	.build();
  (usb_dev)
}
