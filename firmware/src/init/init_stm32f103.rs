use usb_device::bus::UsbBus; 
use usb_device::class_prelude::UsbBusAllocator;

use stm32f1xx_hal::usb::UsbBus as UsbBusHal;
use stm32f1xx_hal::usb::Peripheral;
use stm32f1xx_hal::pac::Peripherals;
use stm32f1xx_hal::pac::GPIOB;
use stm32f1xx_hal::pac::TIM2;
use stm32f1xx_hal::gpio::Pin;
use stm32f1xx_hal::gpio::Output;
use stm32f1xx_hal::gpio::PushPull;
use stm32f1xx_hal::prelude::_stm32_hal_rcc_RccExt;
use stm32f1xx_hal::gpio::GpioExt;

use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::flash::FlashExt;

use static_cell::StaticCell;

pub type BoardUsbBus = UsbBusHal<Peripheral>;

static USB_BUS_ALLOCATOR: StaticCell<UsbBusAllocator<BoardUsbBus>> = StaticCell::new();

pub struct BoardPeripherals {
    pub _gpiob: GPIOB,
    pub _tim2: TIM2,
    pub led: Pin<'C', 6, Output<PushPull>>,
    pub usb_bus: &'static UsbBusAllocator<BoardUsbBus>,
}

pub fn init_rcc() -> BoardPeripherals {
    let dp = Peripherals::take().expect("cannot take peripherals");
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    let clocks = rcc
        .cfgr
        .use_hse(8_u32.MHz())
        .sysclk(72_u32.MHz())
        .pclk1(36_u32.MHz())
        .freeze(&mut flash.acr);

    assert!(clocks.usbclk_valid(), "USB-Takt konnte nicht stabil auf 48MHz konfiguriert werden!");

    let mut gpioa = dp.GPIOA.split();
    let mut gpioc = dp.GPIOC.split();

    let led = gpioc.pc6.into_push_pull_output(&mut gpioc.crl);

    let usb_peripheral = Peripheral {
        usb: dp.USB,
        pin_dm: gpioa.pa11,
        pin_dp: gpioa.pa12,
    };

    let raw = UsbBusHal::new(usb_peripheral);
    let st_bus = USB_BUS_ALLOCATOR.init(raw);

    BoardPeripherals {
        _gpiob: dp.GPIOB,
        _tim2: dp.TIM2,
        led,
        usb_bus: st_bus,
    }
}
