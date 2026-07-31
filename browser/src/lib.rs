#![no_std]

use minicbor::encode::write::Cursor;
use rig_to_usb_logic::cmd::Cmd;
use rig_to_usb_logic::cmd::Cmd::*;

use core::cell::UnsafeCell;

const BUFFER_SIZE: usize = 64;
pub struct Shared<T>(UnsafeCell<T>);

unsafe impl<T> Sync for Shared<T> {}

impl<T> Shared<T> {
    pub const fn new(value: T) -> Self {
        Self(UnsafeCell::new(value))
    }
    pub fn get(&self) -> *mut T {
        self.0.get()
    }
}

#[unsafe(export_name = "PANIC")]
static PANIC:Shared<[u8; BUFFER_SIZE]> = Shared::new([0; BUFFER_SIZE]);
#[unsafe(export_name = "ERR")]
static ERR:Shared<bool> = Shared::new(false);
#[unsafe(export_name = "TXT")]
static TXT:Shared<[u8; BUFFER_SIZE]> = Shared::new([0; BUFFER_SIZE]);
#[unsafe(export_name = "CBOR")]
static CBOR:Shared<[u8; BUFFER_SIZE]> = Shared::new([0; BUFFER_SIZE]);

#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_bootloader() -> usize {
  return_encoded(StartBootLoader)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn panic_cmd() -> usize {
  return_encoded(Panic)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test() -> usize {
  return_encoded(Test)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset() -> usize {
  return_encoded(Reset)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn led(v: bool) -> usize {
    return_encoded(LED{ value: v})
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn led_blink(i: u32) -> usize {
    return_encoded(LEDBlink{ interval: i})
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn radio_on(i: u32) -> usize {
    return_encoded(RadioOn{ time: i})
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn radio_off() -> usize {
    return_encoded(RadioOff)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tx_on(i: u32) -> usize {
    return_encoded(TxOn{ time: i})
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tx_off() -> usize {
    return_encoded(TxOff)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn morse_speed(i: u16) -> usize {
    return_encoded(MorseSpeed{ ditlen: i})
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn morse_send(len: usize) -> usize {
    unsafe{
	let ptr = TXT.get() as *mut u8;
        let slice = core::slice::from_raw_parts_mut(ptr, len);
	return_encoded(MorseSend{ txt: slice})
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn morse_append(len: usize) -> usize {
    unsafe{
	let ptr = TXT.get() as *mut u8;
        let slice = core::slice::from_raw_parts_mut(ptr, len);
	return_encoded(MorseAppend{ txt: slice})
    }
}

fn return_encoded(cmd: Cmd) -> usize {
    unsafe {
    let ptr = CBOR.get() as *mut u8;
    let slice = core::slice::from_raw_parts_mut(ptr, BUFFER_SIZE);

    let mut cursor = Cursor::new(&mut *slice);

    match minicbor::encode(&cmd, &mut cursor) {
        Ok(_) => {
	    return cursor.position()
	}
	Err(_) => {
	    return 0
	}
    };
    };
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let msg_str = info.message().as_str();
    let _msg = match msg_str {
        Some(s) => s,
        None => "WASM Panic aufgetreten (Details in no_std nicht formatierbar)",
    };

    unsafe {
      *ERR.get() = true;
//      host_panic(msg.as_ptr(), msg.len());
    }
    
    core::arch::wasm32::unreachable()
}
