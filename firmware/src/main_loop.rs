use cortex_m::peripheral::DWT;

use usb_device::class_prelude::UsbBus;

use crate::board::Board;
use crate::usb::{MyUsb};
use crate::poll_usb::{poll_usb};
use crate::tasks::{init_tasks, run_pending_tasks};
use crate::hal::IndependentWatchdog;

pub fn main_loop<'a, B: UsbBus>
    (mut watchdog: IndependentWatchdog,
    usb : &mut MyUsb<'a , B>,
    board: &mut Board,
    ) -> !
{
    defmt::info!("Starting Loop");
    let mut tasks = init_tasks();
    let mut clock: u64 = 0;
    let mut old_clk: u32 = DWT::cycle_count();
    loop {
        let clk = DWT::cycle_count();
	clock += (clk.wrapping_sub(old_clk)) as u64;
	old_clk = clk;
	watchdog.feed();
	tasks = poll_usb(board, usb, tasks, clock);
	tasks = run_pending_tasks(board, tasks, clock);
    }
}
