#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;

use stm32g4xx_hal as hal;

use hal::prelude::*;
use hal::stm32::Peripherals;
use hal::rcc;
use hal::rcc::PllSrc;
use hal::rcc::PllMDiv;
use hal::rcc::PllNMul;
use hal::rcc::PllRDiv;

use usbd_hid::{descriptor::generator_prelude::*, hid_class::HIDClass};
use minicbor::{Decode, Encode};
use stm32g4xx_hal::time::RateExtU32;
use stm32g4xx_hal::usb::UsbBus;
use hal::usb::{Peripheral};
use usb_device::prelude::UsbDeviceBuilder;
use usb_device::prelude::UsbVidPid;
use usb_device::prelude::StringDescriptors;
use usb_device::LangID;
use defmt_rtt as _;

#[derive(Decode, Encode, Debug)]
struct MessageIn {
    #[n(0)] id: i32,
    #[n(1)] status: bool,
}

#[entry]
fn main() -> ! {
    defmt::info!("Hello, USB-World!");
    let dp = Peripherals::take().expect("cannot take peripherals");
    let pwr = dp.PWR.constrain().freeze();

    let mut pll_config = stm32g4xx_hal::rcc::PllConfig::default();
    pll_config.mux = PllSrc::HSE(8_u32.MHz());
    pll_config.m = PllMDiv::DIV_2;
    pll_config.n = PllNMul::MUL_60;
    pll_config.r = Some(PllRDiv::DIV_2);

    let config = rcc::Config::new(rcc::SysClockSrc::PLL)
      .pll_cfg(pll_config);

    let mut rcc = dp.RCC.freeze(config, pwr);
    rcc.enable_hsi48();

    let gpioa = dp.GPIOA.split(&mut rcc);
    let usb_dm = gpioa.pa11.into_alternate();
    let usb_dp = gpioa.pa12.into_alternate();

    let usb_peripheral = Peripheral {
        usb: dp.USB,
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
                        if msg.status {
                           defmt::info!("cbor decode success");
                        }
			defmt::info!("cbor decode x");
                    } else
		    {
                           defmt::info!("cbor decode error");			
		    }
                    
                }
                _ => {}
            }
        }
    }
}
