use crate::init::{BoardPeripherals, SYSTEM_CLOCK_MHZ};
use crate::hal::gpio::PinState;
use rig_to_usb_logic::morse::{LineState, next_pin_state };
use rig_to_usb_logic::morse::MorseState::*;

#[derive(Debug)]
pub struct TimeOut {
    pub enabled: bool,
    pub time: u64,
}

pub const TIME_OUT_DISABLED:TimeOut = TimeOut { enabled: false, time: 0};

#[derive(Debug)]
pub struct Tasks {
    pub ping: TimeOut,
    pub led: TimeOut,
    pub led_state: bool,
    pub led_interval: u16,
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

pub fn check_timeout
    (clock: u64,
     t: TimeOut)
     -> (TimeOut, bool)
{
    if t.enabled && clock > t.time {
	return (TimeOut {enabled: false, time: t.time}, true);
    }
    (t, false)
}

pub fn init_tasks() -> Tasks
{
    Tasks {
	ping: TIME_OUT_DISABLED,
	led: schedule_timeout(0, 1000),
	led_state: false,
	led_interval: 1000,
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
    let is_ping;
    (tasks.ping, is_ping) = check_timeout(clock, tasks.ping);
    if is_ping {
	defmt::info!("Ping");
	tasks.ping = schedule_timeout(clock, 10000);
    }
    let is_led;
    (tasks.led, is_led) = check_timeout(clock, tasks.led);
    if is_led {
	tasks.led = schedule_timeout(clock, tasks.led_interval.into());
	tasks.led_state = !tasks.led_state;
	board_peripherals.led.set_state(PinState::from(tasks.led_state));
    }
    let is_tx_off;
    (tasks.tx_off, is_tx_off) = check_timeout(clock, tasks.tx_off);
    if is_tx_off {
	board_peripherals.led.set_state(PinState::from(true));
    }
    let is_morse;
    (tasks.morse_timer, is_morse) = check_timeout(clock, tasks.morse_timer);
    if is_morse && let Some(s) = tasks.morse_state {
	match next_pin_state(s) {
	    None => {
		tasks.morse_state = None;
	    }
	    Some((High, n)) => {
                board_peripherals.led.set_state(PinState::from(false));
		tasks.morse_state = Some(n);
		tasks.morse_timer = next_timeout(tasks.morse_timer, tasks.morse_ditlen.into());
	    }
	    Some((Low, n)) => {
                board_peripherals.led.set_state(PinState::from(true));
		tasks.morse_state = Some(n);
		tasks.morse_timer = next_timeout(tasks.morse_timer, tasks.morse_ditlen.into());
	    }
	}
    }
    tasks
}
