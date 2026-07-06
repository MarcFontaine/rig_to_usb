#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;

use stm32g4xx_hal as hal;

use hal::prelude::*;
use hal::rcc;

use usbd_hid::{hid_class::HIDClass};
use usbd_hid::descriptor::generator_prelude::*;

use stm32g4xx_hal::usb::UsbBus;
use hal::usb::{Peripheral};
use usb_device::prelude::UsbDeviceBuilder;
use usb_device::prelude::UsbVidPid;
use usb_device::prelude::StringDescriptors;
use usb_device::LangID;
use defmt_rtt as _;

mod cmd;
use cmd::{MessageIn, message_example};

mod init;
use init::{init_rcc};

//mod usb;
//use usb::{init_usb};

#[entry]
fn main() -> ! {
    defmt::info!("Hello, USB-World!");
    message_example();

    let mut board_peripherals = init_rcc();

    let gpioa = board_peripherals.gpioa.split(&mut board_peripherals.rcc);
    let usb_dm = gpioa.pa11.into_alternate();
    let usb_dp = gpioa.pa12.into_alternate();

    let usb_peripheral = Peripheral {
        usb: board_peripherals.usb ,
        pin_dm: usb_dm,
        pin_dp: usb_dp,
    };

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

    let mut rx_packet = [0u8; 64];
    defmt::info!("Starting Loop");
    loop {	
        if usb_dev.poll(&mut [&mut hid]) {
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
