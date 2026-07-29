use crate::init::{BoardPeripherals, SYSTEM_CLOCK_MHZ};
use crate::hal::gpio::PinState;
use PinState::*;
use rig_to_usb_logic::morse::{LineState, next_pin_state };
use rig_to_usb_logic::morse::MorseState;

#[derive(Debug)]
pub struct TimeOut {
    pub enabled: bool,
    pub time: u64,
}

impl TimeOut {
    pub fn check_timeout (&mut self, clock: u64) -> bool
    {
	if self.enabled && clock > self.time {
	    self.enabled = false;
	    return true;
	}
	false
    }
}

pub const TIME_OUT_DISABLED:TimeOut = TimeOut { enabled: false, time: 0};

#[derive(Debug)]
pub struct Tasks {
    pub ping: TimeOut,
    pub led: TimeOut,
    pub led_state: bool,
    pub led_interval: u16,
    pub radio_off: TimeOut,
    pub tx_off: TimeOut,
    pub morse_timer: TimeOut,
    pub morse_state: Option<LineState>,
    pub morse_ditlen: u16,
}

pub fn schedule_timeout (clock:u64, ms:u32) -> TimeOut
{
    TimeOut {
	enabled : true,
	time : clock + (ms as u64) * 1000 * (SYSTEM_CLOCK_MHZ as u64),
    }
}

pub fn next_timeout (t: TimeOut, ms:u32) -> TimeOut
{
    TimeOut {
	enabled : true,
	time : t.time + (ms as u64) * 1000 * (SYSTEM_CLOCK_MHZ as u64),
    }
}

pub fn init_tasks() -> Tasks
{
    Tasks {
	ping: TIME_OUT_DISABLED,
	led: schedule_timeout(0, 1000),
	led_state: false,
	led_interval: 1000,
	radio_off: TIME_OUT_DISABLED,
	tx_off: TIME_OUT_DISABLED,
	morse_timer: TIME_OUT_DISABLED,
        morse_state: None,
	morse_ditlen: 60, // 60ms -> 20WPM
    }
}

pub fn run_pending_tasks
    (board_peripherals: &mut BoardPeripherals,
     mut tasks: Tasks,
     clock: u64
    )
    -> Tasks
{
    if tasks.ping.check_timeout(clock) {
	defmt::info!("Ping");
	tasks.ping = schedule_timeout(clock, 10000);
    }
    if tasks.led.check_timeout(clock) {
	tasks.led = schedule_timeout(clock, tasks.led_interval.into());
	tasks.led_state = !tasks.led_state;
	board_peripherals.led.set(tasks.led_state);
    }
    if tasks.radio_off.check_timeout(clock) {
	board_peripherals.on_off.set_state(High);
	board_peripherals.hochschalten.set_state(High);
    }
    if tasks.tx_off.check_timeout(clock) {
	board_peripherals.hochschalten.set_state(High);
    }
    if tasks.morse_timer.check_timeout(clock) && let Some(s) = tasks.morse_state {
	match next_pin_state(s) {
	    None => {
		tasks.morse_state = None;
	    }
	    Some((MorseState::High, n)) => {
                board_peripherals.hochschalten.set_state(Low);
		tasks.morse_state = Some(n);
		tasks.morse_timer = next_timeout(tasks.morse_timer, tasks.morse_ditlen.into());
	    }
	    Some((MorseState::Low, n)) => {
                board_peripherals.hochschalten.set_state(High);
		tasks.morse_state = Some(n);
		tasks.morse_timer = next_timeout(tasks.morse_timer, tasks.morse_ditlen.into());
	    }
	}
    }
    tasks
}
