#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;

use usbd_hid::{hid_class::HIDClass};
use usbd_hid::descriptor::generator_prelude::*;

use stm32g4xx_hal::usb::UsbBus;
use usb_device::prelude::UsbDeviceBuilder;
use usb_device::prelude::UsbVidPid;
use usb_device::prelude::StringDescriptors;
use usb_device::LangID;
use defmt_rtt as _;

use rig_to_usb::firmware::cmd::{message_example};
use rig_to_usb::firmware::init::{init_rcc};
use rig_to_usb::firmware::main_loop::{main_loop};

#[entry]
fn main() -> ! {
    defmt::info!("Hello, USB-World!");
    message_example();

    let (usb_peripheral, mut board_peripherals) = init_rcc();

    let usb_bus = UsbBus::new(usb_peripheral);

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

    main_loop(&mut board_peripherals, &mut usb_dev, &mut hid);
}
