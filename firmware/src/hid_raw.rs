#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;

use defmt_rtt as _;

use rig_to_usb::init::{init_rcc};
use rig_to_usb::main_loop::{main_loop};

use rig_to_usb::usb::{init_usb};
#[entry]
fn main() -> ! {
    defmt::info!("Hello, USB-World!");

    let mut board_peripherals = init_rcc();

    let (mut usb_dev, mut hid) = init_usb(board_peripherals.usb_bus);

    main_loop(&mut board_peripherals, &mut usb_dev, &mut hid);
}
