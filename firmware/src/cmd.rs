use defmt_rtt as _;
use crate::bootloader::{jump_to_bootloader};
use crate::board::Board;
use crate::tasks::{Tasks, schedule_timeout, TIME_OUT_DISABLED};
use cortex_m::peripheral::SCB;

use rig_to_usb_logic::cmd::Cmd;
use rig_to_usb_logic::cmd::Cmd::*;
use rig_to_usb_logic::morse::LineState;

pub fn decode_and_run(
    board: &mut Board,
    mut tasks: Tasks,
    packet: &[u8],
    clock: u64,
) -> Tasks
{
    let mut decoder = minicbor::Decoder::new(&packet);
    if let Ok(cmd) = decoder.decode::<Cmd>() {
	defmt::info!("cbor decode message: {}", cmd);
	tasks = run_cmd(board, tasks, cmd, clock);
    } else
    {
    defmt::info!("cbor decode error");
    defmt::info!("cbor decode daten: {=[u8]:x}", packet);
    defmt::info!("cbor decode daten: {}", packet);
    }
    tasks
}

pub fn run_cmd(
    board: &mut Board,
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
	    board.led_set(value);
	    tasks.led = TIME_OUT_DISABLED;
	}
	LEDBlink{ interval } => {
	    tasks.led = schedule_timeout(clock, interval.into());
	}
	RadioOn{ time } => {
	    board.radio_on();
	    tasks.radio_off = schedule_timeout(clock, time);
	}
	RadioOff => {
	    board.radio_off();
	}
	TxOn{ time } => {
	    board.tx_on();
	    tasks.tx_off = schedule_timeout(clock, time);
	}
	TxOff => {
	    board.tx_off();
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
