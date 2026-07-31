#![cfg_attr(not(feature = "std-json"), no_std)]

#[cfg(feature = "defmt")]
pub use defmt::{info, warn, error};

#[cfg(not(feature = "defmt"))]
#[macro_export]
macro_rules! info {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {};
}

#[cfg(not(feature = "defmt"))]
#[macro_export]
macro_rules! warn {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {};
}
#[cfg(not(feature = "defmt"))]
#[macro_export]
macro_rules! error {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {};
}

#[cfg(feature = "std-json")]
extern crate std;

#[cfg(feature = "std-json")]
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        $crate::std::println!($($arg)*);
    };
}

#[cfg(not(feature = "std-json"))]
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {};
}

#[cfg(feature = "std-json")]
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::std::print!($($arg)*);
    };
}

#[cfg(not(feature = "std-json"))]
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {};
}

pub mod cmd;
pub mod morse;
pub mod test;
