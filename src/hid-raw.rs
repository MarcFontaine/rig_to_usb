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

// --- 1. MINICBOR STRUKTUR ---
// Ein einfaches CBOR-Array mit zwei Feldern (int und bool)
#[derive(Decode, Encode, Debug)]
struct HelloMessage {
    #[n(0)] id: i32,
    #[n(1)] status: bool,
}

#[entry]
fn main() -> ! {
    defmt::info!("Hello, USB-World!");
    let dp = Peripherals::take().expect("cannot take peripherals");
    defmt::info!("Hello, USB-World!2");
    let pwr = dp.PWR.constrain().freeze();
    defmt::info!("Hello, USB-World!3");

    let mut pll_config = stm32g4xx_hal::rcc::PllConfig::default();
    pll_config.mux = PllSrc::HSE(8_u32.MHz());
    pll_config.m = PllMDiv::DIV_2;
    pll_config.n = PllNMul::MUL_85;
    pll_config.r = Some(PllRDiv::DIV_2);
    defmt::info!("Hello, USB-World!4");    

    let config = rcc::Config::new(rcc::SysClockSrc::PLL)
      .pll_cfg(pll_config);
    defmt::info!("Hello, USB-World!5");

    let mut rcc = dp.RCC.freeze(config, pwr);
    rcc.enable_hsi48();

    defmt::info!("Hello, USB-World!6");
    let gpioa = dp.GPIOA.split(&mut rcc);
    let usb_dm = gpioa.pa11.into_alternate();
    let usb_dp = gpioa.pa12.into_alternate();

    defmt::info!("Hello, USB-World!7");
    let usb_peripheral = Peripheral {
        usb: dp.USB,
        pin_dm: usb_dm,
        pin_dp: usb_dp,
    };

    defmt::info!("Hello, USB-World!8");
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
    defmt::info!("Hello, USB-World!9");
    let mut hid = HIDClass::new(&usb_bus, CustomBidirectionalReport::desc(), 10); // 10ms Polling-Intervall

    // Generisches USB-Gerät erstellen
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x1209, 0x0001))
        .strings(&[
           StringDescriptors::new(LangID::EN)
                .manufacturer("RustEmbedded")
                .product("CBOR-RawHID-HelloWorld")
                .serial_number("123456")
        ])
	.expect("Cannot set USB String Descriptors") 
	.device_class(0x00)
	.build();
    defmt::info!("Hello, USB-World!10");
    let mut rx_packet = [0u8; 64];
    defmt::info!("Hello, USB-World!11");
    loop {	
        // USB-Zustandsmaschine im Polling-Verfahren abfragen
        if usb_dev.poll(&mut [&mut hid]) {
            // Prüfen, ob ein neues 64-Byte-Paket vom WebHID-Browser eingetroffen ist
            match hid.pull_raw_output(&mut rx_packet) {
                Ok(bytes_read) if bytes_read > 0 => {
                    
                    // --- CBOR DEKODIERUNG ---
                    // Wir füttern den minicbor-Decoder mit den empfangenen USB-Bytes.
                    // Er liest selbstbegrenzend nur die Struktur aus und ignoriert Padding-Nullen.
                    let mut decoder = minicbor::Decoder::new(&rx_packet[..bytes_read]);
                    
                    if let Ok(msg) = decoder.decode::<HelloMessage>() {
                        // Daten erfolgreich extrahiert! 
                        // msg.id und msg.status stehen jetzt zur Verfügung.
                        
                        // Ein visuelles Feedback zur Bestätigung (z.B. LED umschalten)
                        if msg.status {
                            // Hier Code zum Anschalten einer LED einfügen
                        }
                    }
                    
                }
                _ => {}
            }
        }
    }
}
