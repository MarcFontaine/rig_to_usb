#![no_std]

pub mod bootloader;
pub mod cmd;
pub mod init;
pub mod usb;
pub mod main_loop;

pub mod hal {
  pub use stm32g4xx_hal::stm32::{Peripherals};
  pub use stm32g4xx_hal::rcc;
  pub use stm32g4xx_hal::rcc::{PllSrc, PllMDiv, PllNMul, PllRDiv};
  pub use stm32g4xx_hal::time::RateExtU32;
  pub use stm32g4xx_hal::gpio::{ GpioExt, Pin, Output};
  pub use stm32g4xx_hal::rcc::RccExt;
  pub use stm32g4xx_hal::pwr::PwrExt;
  pub use stm32g4xx_hal::stm32::TIM2;
  pub use stm32g4xx_hal::usb::Peripheral;
  pub use stm32g4xx_hal::stm32::GPIOB;
  pub use stm32g4xx_hal::usb::UsbBus;
  pub use stm32g4xx_hal::pac;
}
