#![no_std]
#![no_main]

//TODO: conditional import on dev/release profile
use panic_probe as _; // exit with error
// use panic_halt as _; loops forever
use cortex_m_rt::entry;

use defmt_rtt as _;

use rig_to_usb::init::{init_rcc};
use rig_to_usb::main_loop::{main_loop};

use rig_to_usb::usb::{init_usb};
use rig_to_usb::bootloader::{set_jump_to_bootloader_flag};
#[entry]
fn main() -> ! {
    set_jump_to_bootloader_flag();
    defmt::info!("Hello, USB-World!");

    let (watchdog, usb_bus, mut board) = init_rcc();
    let mut my_usb = init_usb(usb_bus);

    main_loop(watchdog, &mut my_usb, &mut board,);
}
