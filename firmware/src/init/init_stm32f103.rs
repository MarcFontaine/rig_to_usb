use stm32f1xx_hal::rcc;

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
    pub led: Pin<'C',13, Output<PushPull>>,
    pub usb_bus: &'static UsbBusAllocator<BoardUsbBus>,
}

pub fn init_rcc() -> BoardPeripherals {
    let dp = Peripherals::take().expect("cannot take peripherals");
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()).sysclk(48.MHz()).pclk1(24.MHz()),
        &mut flash.acr,
    );

    let gpioa = dp.GPIOA.split(&mut rcc);
    let mut gpioc = dp.GPIOC.split(&mut rcc);

    let led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);

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
