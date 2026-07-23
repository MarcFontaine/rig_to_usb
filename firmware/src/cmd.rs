use defmt_rtt as _;
use crate::hal::gpio::PinState;

//use crate::bootloader::jump_to_st_bootloader;
use crate::init::{BoardPeripherals};
//use crate::test::{test};
use crate::tasks::{Tasks, schedule_timeout};

use rig_to_usb_logic::cmd::Cmd;
use rig_to_usb_logic::cmd::Cmd::*;

pub fn decode_and_run(
    board_peripherals: &mut BoardPeripherals,
    mut tasks: Tasks,
    packet: &[u8]
) -> Tasks
{
    let mut decoder = minicbor::Decoder::new(&packet);
    if let Ok(cmd) = decoder.decode::<Cmd>() {
	defmt::info!("cbor decode message: {}", cmd);
	tasks = run_cmd(board_peripherals, tasks, cmd);
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
    cmd: Cmd
)
    -> Tasks
{
    match cmd {
	StartBootLoader() => {} //jump_to_st_bootloader();}
	LED{ value } => {board_peripherals.led.set_state(PinState::from(value));}
	Test() => { }
	TxOn{ time } => {
	    board_peripherals.led.set_state(PinState::from(false));
	    tasks.tx_off = schedule_timeout(time);
	}
    }
    tasks
}
