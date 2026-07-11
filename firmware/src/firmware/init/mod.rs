use stm32g4xx_hal::stm32::{Peripherals};
use stm32g4xx_hal::rcc;
use stm32g4xx_hal::rcc::{PllSrc, PllMDiv, PllNMul, PllRDiv};
use stm32g4xx_hal::time::RateExtU32;
use stm32g4xx_hal::gpio::{ GpioExt, Pin, Output};
use stm32g4xx_hal::rcc::RccExt;
use stm32g4xx_hal::pwr::PwrExt;
use stm32g4xx_hal::stm32::TIM2;
use stm32g4xx_hal::usb::Peripheral;
use stm32g4xx_hal::stm32::GPIOB;

pub struct BoardPeripherals {
    pub _gpiob: GPIOB,
    pub _tim2: TIM2,
    pub led: Pin<'C', 6, Output>,
}

pub fn init_rcc() -> (
    Peripheral<stm32g4xx_hal::gpio::Pin<'A', 11, stm32g4xx_hal::gpio::Alternate<14>>, stm32g4xx_hal::gpio::Pin<'A', 12, stm32g4xx_hal::gpio::Alternate<14>>>,
    BoardPeripherals)
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
    let led = dp.GPIOC.split(&mut rcc).pc6.into_push_pull_output();

    let gpioa = dp.GPIOA.split(&mut rcc);
    let usb_dm = gpioa.pa11.into_alternate();
    let usb_dp = gpioa.pa12.into_alternate();

    let usb_peripheral = Peripheral {
        usb: dp.USB,
        pin_dm: usb_dm,
        pin_dp: usb_dp,
    };
    (usb_peripheral,
    BoardPeripherals {
        _gpiob: dp.GPIOB,
        _tim2: dp.TIM2,
	led: led
    }
    )
}
