use cortex_m::asm::delay;

use crate::hal::{*};
use usb_device::class_prelude::UsbBusAllocator;
use static_cell::StaticCell;

pub const SYSTEM_CLOCK_MHZ: u32 = 150;
use crate::pins;

pub type BoardUsbBus = UsbBus<Peripheral<Pin<'A', 11, Alternate<14>>, Pin<'A', 12, Alternate<14>>>>;
static USB_BUS_ALLOCATOR: StaticCell<UsbBusAllocator<BoardUsbBus>> = StaticCell::new();

pub struct BoardPeripherals {
    pub watchdog: IndependentWatchdog,
    pub led: pins::LED,
    pub usb_bus: &'static UsbBusAllocator<BoardUsbBus>,
}

pub fn init_rcc() -> BoardPeripherals
{
    let dp = Peripherals::take().expect("cannot take peripherals");

    let pwr = dp.PWR.constrain().freeze();
 
    let mut pll_config = stm32g4xx_hal::rcc::PllConfig::default();
    pll_config.mux = PllSrc::HSE(8_u32.MHz());
    pll_config.m = PllMDiv::DIV_2;
    pll_config.n = PllNMul::MUL_75;
    pll_config.r = Some(PllRDiv::DIV_2);

    let config = rcc::Config::new(rcc::SysClockSrc::PLL)
      .pll_cfg(pll_config);

    let mut rcc = dp.RCC.freeze(config, pwr);
    rcc.enable_hsi48();

    // TODO: test watchdog
    let mut watchdog = IndependentWatchdog::new(dp.IWDG);
    watchdog.start(10000_u32.millis());
    delay( 500*SYSTEM_CLOCK_MHZ );
    watchdog.start(10000_u32.millis());

    let gpioa = dp.GPIOA.split(&mut rcc);
    let usb_dm = gpioa.pa11.into_alternate();
    let usb_dp = gpioa.pa12.into_alternate();

    let usb_peripheral = Peripheral {
        usb: dp.USB,
        pin_dm: usb_dm,
        pin_dp: usb_dp,
    };
    let raw = UsbBus::new(usb_peripheral);
    let st_bus = USB_BUS_ALLOCATOR.init(raw);

    BoardPeripherals {
        watchdog: watchdog,
	led: dp.GPIOC.split(&mut rcc).pc6.into_push_pull_output(),
	usb_bus: st_bus
    }
}
