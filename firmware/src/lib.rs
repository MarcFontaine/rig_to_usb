#![no_std]

pub mod stm32_dfu_bootloader;
pub use stm32_dfu_bootloader as bootloader;
pub mod cmd;
pub mod init;
pub mod usb;
pub mod poll_usb;
pub mod hid_push;
pub mod main_loop;
pub mod test;
pub mod tasks;
pub mod board;
pub mod uart_tx;
pub mod uart_iterator;

#[cfg(feature = "stm32g474")]
pub mod pins_we_act;

#[cfg(feature = "stm32g474")]
pub use pins_we_act as pins;

#[cfg(feature = "stm32g474")]
pub mod hal {
  pub use stm32g4xx_hal::stm32::{Peripherals};
  pub use stm32g4xx_hal::rcc;
  pub use stm32g4xx_hal::rcc::{PllSrc, PllMDiv, PllNMul, PllRDiv};
  pub use stm32g4xx_hal::time::RateExtU32;
  pub use stm32g4xx_hal::gpio::{ GpioExt, Pin, Output, PushPull};
  pub use stm32g4xx_hal::rcc::RccExt;
  pub use stm32g4xx_hal::pwr::PwrExt;
  pub use stm32g4xx_hal::stm32::TIM2;
  pub use stm32g4xx_hal::usb::Peripheral;
  pub use stm32g4xx_hal::stm32::GPIOB;
  pub use stm32g4xx_hal::usb::UsbBus;
  pub use stm32g4xx_hal::gpio::Alternate;
  pub use stm32g4xx_hal::gpio;
  pub use stm32g4xx_hal::pac;
  pub use stm32g4xx_hal::time::ExtU32;
  pub use stm32g4xx_hal::independent_watchdog::IndependentWatchdog;
}

#[cfg(feature = "stm32f103")]
pub mod hal {
    pub use stm32f1xx_hal;
    pub use stm32f1xx_hal::gpio;
    pub use stm32f1xx_hal::rcc;
    pub use stm32f1xx_hal::pac;
    pub use stm32f1xx_hal::gpio::{ GpioExt, Pin, Output, PushPull};
    pub use stm32f1xx_hal::watchdog::IndependentWatchdog;
    pub use stm32f1xx_hal::serial;
    pub use stm32f1xx_hal::time;
    pub use stm32f1xx_hal::dma;
}
