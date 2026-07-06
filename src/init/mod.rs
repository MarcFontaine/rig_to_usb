use stm32g4xx_hal as hal;

use hal::prelude::*;
use hal::stm32::{Peripherals};
use hal::rcc;
use hal::rcc::PllSrc;
use hal::rcc::PllMDiv;
use hal::rcc::PllNMul;
use hal::rcc::PllRDiv;
use stm32g4xx_hal::time::RateExtU32;
use crate::rcc::Rcc;
use crate::hal::stm32::GPIOA;
use crate::hal::stm32::GPIOB;
use crate::hal::stm32::TIM2;
use crate::hal::stm32::USB;

pub struct BoardPeripherals {
    pub rcc: Rcc,
    pub gpioa: GPIOA,
    pub _gpiob: GPIOB,
    pub _tim2: TIM2,
    pub usb: USB,    
}

pub fn init_rcc() -> BoardPeripherals {
    let dp = Peripherals::take().expect("cannot take peripherals");

    let pwr = dp.PWR.constrain().freeze();
 
    let mut pll_config = stm32g4xx_hal::rcc::PllConfig::default();
    pll_config.mux = PllSrc::HSE(8_u32.MHz());
    pll_config.m = PllMDiv::DIV_2;
    pll_config.n = PllNMul::MUL_75;
    pll_config.r = Some(PllRDiv::DIV_2);

    let config = rcc::Config::new(rcc::SysClockSrc::PLL)
      .pll_cfg(pll_config);

    let rcc = dp.RCC.freeze(config, pwr);
    rcc.enable_hsi48();
    BoardPeripherals {
        rcc,
        gpioa: dp.GPIOA,
        _gpiob: dp.GPIOB,
        _tim2: dp.TIM2,
        usb: dp.USB,	
    }
}
