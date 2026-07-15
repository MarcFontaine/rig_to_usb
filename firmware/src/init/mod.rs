#[cfg(feature = "stm32g474")]
mod init_stm32g474;

#[cfg(feature = "stm32f103")]
mod init_stm32f103;

#[cfg(feature = "stm32g474")]
pub use crate::init::init_stm32g474::{BoardPeripherals, BoardUsbBus, init_rcc};

#[cfg(feature = "stm32f103")]
pub use crate::init::init_stm32f103::{BoardPeripherals, BoardUsbBus, init_rcc};
