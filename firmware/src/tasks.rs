use crate::init::{BoardPeripherals};
use cortex_m::peripheral::DWT;

#[derive(Debug)]
pub struct Tasks {
    pub ping: u32
}

pub fn init_tasks() -> Tasks
{
    Tasks {
	ping: DWT::cycle_count().wrapping_add(10*72000000)
    }
}

pub fn run_pending_tasks
    (_board_peripherals: &mut BoardPeripherals
    , mut tasks : Tasks)
    -> Tasks
{
    if tasks.ping.wrapping_sub(DWT::cycle_count()) > (1<<31) {
	defmt::info!("Ping");
	tasks.ping = DWT::cycle_count().wrapping_add(10*72000000);
    }
    tasks
}
