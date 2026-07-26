use stm32f1xx_hal::rcc;

use usb_device::class_prelude::UsbBusAllocator;

use stm32f1xx_hal::usb::UsbBus as UsbBusHal;
use stm32f1xx_hal::usb::Peripheral;
use stm32f1xx_hal::pac::Peripherals;
use stm32f1xx_hal::pac::GPIOB;
use stm32f1xx_hal::gpio::Pin;
use stm32f1xx_hal::gpio::Output;
use stm32f1xx_hal::gpio::PushPull;
use stm32f1xx_hal::prelude::_stm32_hal_rcc_RccExt;
use stm32f1xx_hal::gpio::GpioExt;

use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::prelude::_fugit_ExtU32;
use stm32f1xx_hal::flash::FlashExt;
use stm32f1xx_hal::watchdog::IndependentWatchdog;

use static_cell::StaticCell;

pub const SYSTEM_CLOCK_MHZ: u32 = 72;

pub type BoardUsbBus = UsbBusHal<Peripheral>;

static USB_BUS_ALLOCATOR: StaticCell<UsbBusAllocator<BoardUsbBus>> = StaticCell::new();

pub struct BoardPeripherals {
    pub watchdog: IndependentWatchdog,
    pub _gpiob: GPIOB,
    pub led: Pin<'C',13, Output<PushPull>>,
    pub usb_bus: &'static UsbBusAllocator<BoardUsbBus>,
}

pub fn init_rcc() -> BoardPeripherals {
    let mut cp = cortex_m::Peripherals::take().unwrap();
    cp.DCB.enable_trace();
    cp.DWT.enable_cycle_counter();

    let dp = Peripherals::take().expect("cannot take peripherals");
    let mut watchdog = IndependentWatchdog::new(dp.IWDG);

    watchdog.start(10000_u32.millis());

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()).sysclk(SYSTEM_CLOCK_MHZ.MHz()).pclk1(36.MHz()),
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
	watchdog: watchdog,
        _gpiob: dp.GPIOB,
        led,
        usb_bus: st_bus,
    }
}
