use usb_device::class_prelude::UsbBus;

use crate::init::{BoardPeripherals};
use crate::usb::{MyUsb};
use crate::poll_usb::{poll_usb};
use crate::tasks::{init_tasks, run_pending_tasks};

pub fn main_loop<'a, B: UsbBus>
    (board_peripherals: &mut BoardPeripherals
    ,my_usb : &mut MyUsb<'a , B>
    ) -> !
{
    let mut tasks = init_tasks();
    defmt::info!("Starting Loop");
    loop {
	tasks = poll_usb(board_peripherals, my_usb, tasks);
	tasks = run_pending_tasks(board_peripherals, tasks);
    }
}
