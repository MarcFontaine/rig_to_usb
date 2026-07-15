use defmt_rtt as _;
use crate::hal::gpio::PinState;

//use crate::bootloader::jump_to_st_bootloader;
use crate::init::{BoardPeripherals};

use rig_to_usb_logic::cmd::Cmd;
use rig_to_usb_logic::cmd::Cmd::*;

pub fn decode_and_run(
    board_peripherals: &mut BoardPeripherals,
    packet: &[u8]
)
{
    let mut decoder = minicbor::Decoder::new(&packet);
    if let Ok(cmd) = decoder.decode::<Cmd>() {
	defmt::info!("cbor decode message: {}", cmd);
	run_cmd(board_peripherals,cmd);
    } else
    {
    defmt::info!("cbor decode error");
    defmt::info!("cbor decode daten: {=[u8]:x}", packet);
    defmt::info!("cbor decode daten: {}", packet);
    }
}

pub fn run_cmd(
    board_peripherals: &mut BoardPeripherals,
    cmd: Cmd
)
{
    match cmd {
	StartBootLoader() => {} //jump_to_st_bootloader();}
	LED{ value } => {board_peripherals.led.set_state(PinState::from(value));}
	Error { code: _, message: _ } => {}
	Success { value:_ } => {} //jump_to_st_bootloader();}
    }
}
