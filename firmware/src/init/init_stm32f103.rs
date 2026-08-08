use cortex_m::asm::delay;

use usb_device::class_prelude::UsbBusAllocator;

use stm32f1xx_hal::rcc;
use stm32f1xx_hal::usb::UsbBus as UsbBusHal;
use stm32f1xx_hal::usb::Peripheral;
use stm32f1xx_hal::pac::Peripherals;
use stm32f1xx_hal::gpio::PinState;
use stm32f1xx_hal::prelude::_stm32_hal_rcc_RccExt;
use stm32f1xx_hal::gpio::GpioExt;
use stm32f1xx_hal::prelude::_fugit_RateExtU32;
use stm32f1xx_hal::prelude::_fugit_ExtU32;
use stm32f1xx_hal::flash::FlashExt;
use stm32f1xx_hal::watchdog::IndependentWatchdog;

use crate::hal;
use stm32f1xx_hal::dma::DmaExt;

use static_cell::StaticCell;

use crate::board::{Board,TxState};

pub const SYSTEM_CLOCK_MHZ: u32 = 72;

pub type BoardUsbBus = UsbBusHal<Peripheral>;

static USB_BUS_ALLOCATOR: StaticCell<UsbBusAllocator<BoardUsbBus>> = StaticCell::new();

pub fn init_rcc() ->
    (
	IndependentWatchdog,
	&'static UsbBusAllocator<BoardUsbBus>,
	Board
    )
{
    let mut cp = cortex_m::Peripherals::take().unwrap();
    cp.DCB.enable_trace();
    cp.DWT.enable_cycle_counter();

    let dp = Peripherals::take().expect("cannot take peripherals");

    // bugfix !! start watchdog twice
    let mut watchdog = IndependentWatchdog::new(dp.IWDG);
    watchdog.start(10000_u32.millis());
    delay( 500*SYSTEM_CLOCK_MHZ );
    watchdog.start(10000_u32.millis());

    let mut flash = dp.FLASH.constrain();
    let mut clocks = dp.RCC.freeze(
        rcc::Config::hse(8.MHz()).sysclk(SYSTEM_CLOCK_MHZ.MHz()).pclk1(36.MHz()),
        &mut flash.acr,
    );

    let mut gpioa = dp.GPIOA.split(&mut clocks);
    let mut gpioc = dp.GPIOC.split(&mut clocks);

    gpioa.pa12.as_push_pull_output_with_state(
	&mut gpioa.crh,
	PinState::Low,
        |_pin| { delay( 50*1000*SYSTEM_CLOCK_MHZ ) }
    );

    let usb_peripheral = Peripheral {
        usb: dp.USB,
        pin_dm: gpioa.pa11,
        pin_dp: gpioa.pa12,
    };

    let raw = UsbBusHal::new(usb_peripheral);
    let st_bus = USB_BUS_ALLOCATOR.init(raw);

    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    let serial = hal::serial::Serial::new(
        dp.USART1,
        (tx, rx),
        hal::serial::Config::default().baudrate(hal::time::Bps(9600)),
        &mut clocks,
    );

    let (tx_channel, _rx_channel) = serial.split();

    let dma = dp.DMA1.split(&mut clocks);

    let mut board = Board {
        led: gpioc.pc13.into_push_pull_output(&mut gpioc.crh),
        hochschalten: gpioc.pc14.into_push_pull_output(&mut gpioc.crh),
	on_off: gpioc.pc15.into_push_pull_output(&mut gpioc.crh),
	cat_tx: TxState::Ready(tx_channel.with_dma(dma.4))
    };
    board.radio_off();
    board.tx_off();
    (watchdog,
     st_bus,
     board
    )
}
