use crate::init::{BoardPeripherals, SYSTEM_CLOCK_MHZ};
use crate::hal::gpio::PinState;
use cortex_m::peripheral::DWT;
use rig_to_usb_logic::morse::{LineState, next_pin_state };
use rig_to_usb_logic::morse::MorseState::*;

#[derive(Debug)]
pub struct TimeOut {
    pub enabled: bool,
    pub time: u32,
}

pub const TIME_OUT_DISABLED:TimeOut = TimeOut { enabled: false, time: 0};

#[derive(Debug)]
pub struct Tasks {
    pub ping: TimeOut,
    pub tx_off: TimeOut,
    pub morse_timer: TimeOut,
    pub morse_state: Option<LineState>,
}

pub fn schedule_timeout (ms:u32) -> TimeOut
{
    TimeOut {
	enabled : true,
	time : DWT::cycle_count().wrapping_add(ms * 1000 * SYSTEM_CLOCK_MHZ),
    }
}

pub fn next_timeout (t: TimeOut, ms:u32) -> TimeOut
{
    TimeOut {
	enabled : true,
	time : t.time.wrapping_add(ms * 1000 * SYSTEM_CLOCK_MHZ),
    }
}

pub fn check_timeout
    (t: TimeOut)
     -> (TimeOut, bool)
{
    if t.enabled && t.time.wrapping_sub(DWT::cycle_count()) > (1<<31) {
	return (TimeOut {enabled: false, time: t.time}, true);
    }
    (t, false)
}

pub fn init_tasks() -> Tasks
{
    Tasks {
	ping: TIME_OUT_DISABLED,
	tx_off: TIME_OUT_DISABLED,
	morse_timer: TIME_OUT_DISABLED,
        morse_state: None,
    }
}

pub fn run_pending_tasks
    (board_peripherals: &mut BoardPeripherals
    , mut tasks : Tasks)
    -> Tasks
{
    let is_ping;
    (tasks.ping, is_ping) = check_timeout(tasks.ping);
    if is_ping {
	defmt::info!("Ping");
	tasks.ping = schedule_timeout(10000);
    }
    let is_tx_off;
    (tasks.tx_off, is_tx_off) = check_timeout(tasks.tx_off);
    if is_tx_off {
	board_peripherals.led.set_state(PinState::from(true));
    }
    let is_morse;
    (tasks.morse_timer, is_morse) = check_timeout(tasks.morse_timer);
    if is_morse && let Some(s) = tasks.morse_state {
	match next_pin_state(s) {
	    None => {
		tasks.morse_state = None;
	    }
	    Some((High, n)) => {
                board_peripherals.led.set_state(PinState::from(false));
		tasks.morse_state = Some(n);
		tasks.morse_timer = next_timeout(tasks.morse_timer,100);
	    }
	    Some((Low, n)) => {
                board_peripherals.led.set_state(PinState::from(true));
		tasks.morse_state = Some(n);
		tasks.morse_timer = next_timeout(tasks.morse_timer,100);
	    }
	}
    }
    tasks
}
