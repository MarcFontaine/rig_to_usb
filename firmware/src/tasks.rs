use crate::init::{BoardPeripherals, SYSTEM_CLOCK_MHZ};
use cortex_m::peripheral::DWT;

#[derive(Debug)]
pub struct TimeOut {
    pub enabled: bool,
    pub time: u32,
}

#[derive(Debug)]
pub struct Tasks {
    pub ping: TimeOut
}

pub fn schedule_timeout (ms:u32) -> TimeOut
{
    TimeOut {
	enabled : true,
	time : DWT::cycle_count().wrapping_add(ms*1000*SYSTEM_CLOCK_MHZ),
    }
}

pub fn check_timeout
    (t: TimeOut)
     -> (TimeOut, bool)
{
    if t.enabled && t.time.wrapping_sub(DWT::cycle_count()) > (1<<31) {
	return (TimeOut {enabled: false, time:0}, true);
    }
    (t, false)
}

pub fn init_tasks() -> Tasks
{
    Tasks {
	ping: schedule_timeout(10000)
    }
}


pub fn run_pending_tasks
    (_board_peripherals: &mut BoardPeripherals
    , mut tasks : Tasks)
    -> Tasks
{
    let is_ping;
    (tasks.ping, is_ping) = check_timeout(tasks.ping);
    if is_ping {
	defmt::info!("Ping");
	tasks.ping = schedule_timeout(10000);
    }
    tasks
}
