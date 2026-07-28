use defmt_rtt as _;
use crate::hal::gpio::PinState;

use crate::bootloader::{jump_to_bootloader};
use crate::init::{BoardPeripherals};
use crate::tasks::{Tasks, schedule_timeout, TIME_OUT_DISABLED};
use cortex_m::peripheral::SCB;

use rig_to_usb_logic::cmd::Cmd;
use rig_to_usb_logic::cmd::Cmd::*;
use rig_to_usb_logic::morse::LineState;

pub fn decode_and_run(
    board_peripherals: &mut BoardPeripherals,
    mut tasks: Tasks,
    packet: &[u8],
    clock: u64,
) -> Tasks
{
    let mut decoder = minicbor::Decoder::new(&packet);
    if let Ok(cmd) = decoder.decode::<Cmd>() {
	defmt::info!("cbor decode message: {}", cmd);
	tasks = run_cmd(board_peripherals, tasks, cmd, clock);
    } else
    {
    defmt::info!("cbor decode error");
    defmt::info!("cbor decode daten: {=[u8]:x}", packet);
    defmt::info!("cbor decode daten: {}", packet);
    }
    tasks
}

pub fn run_cmd(
    board_peripherals: &mut BoardPeripherals,
    mut tasks: Tasks,
    cmd: Cmd,
    clock: u64,
)
    -> Tasks
{
    match cmd {
	Panic => panic!("Panic command test"),
	Test => { }
	Reset => {
           SCB::sys_reset()
	}
	StartBootLoader => { jump_to_bootloader(); }
	LED{ value } => {
	    board_peripherals.led.set_state(PinState::from(value));
	    tasks.led = TIME_OUT_DISABLED;
	}
	LEDBlink{ interval } => {
	    tasks.led = schedule_timeout(clock, interval.into());
	}
	TxOn{ time } => {
	    board_peripherals.led.set_state(PinState::from(false));
	    tasks.tx_off = schedule_timeout(clock, time);
	}
	MorseSpeed{ ditlen } => { tasks.morse_ditlen = ditlen }
	MorseSend{ txt } => {
	    tasks.morse_state = Some(LineState::init(txt));
            tasks.morse_timer = schedule_timeout(clock, tasks.morse_ditlen.into());
	}
	MorseAppend{ txt } => match &mut tasks.morse_state {
	    None => {
		tasks.morse_state = Some(LineState::init(txt));
		tasks.morse_timer = schedule_timeout(clock, tasks.morse_ditlen.into());
	    }
	    Some(s) => {
		s.append(txt);
	    }
	}
    }
    tasks
}
