#![no_std]

use minicbor::encode::write::Cursor;
use rig_to_usb_logic::cmd::Cmd;
use rig_to_usb_logic::cmd::Cmd::*;

use core::cell::UnsafeCell;

const BUFFER_SIZE: usize = 64;
// 1. Wir bauen einen dünnen Wrapper um UnsafeCell
struct SyncBuffer(UnsafeCell<[u8; BUFFER_SIZE]>);

// 2. Wir versprechen Rust manuell, dass dieser Typ "Sync" (thread-sicher) ist
unsafe impl Sync for SyncBuffer {}

// 3. Jetzt akzeptiert der Compiler die globale Variable problemlos
static SHARED_BUFFER: SyncBuffer = SyncBuffer(UnsafeCell::new([0; BUFFER_SIZE]));

#[unsafe(no_mangle)]
pub extern "C" fn get_buffer_ptr() -> *mut u8 {
    SHARED_BUFFER.0.get() as *mut u8
}


#[unsafe(no_mangle)]
pub extern "C" fn get_buffer_capacity() -> usize {
    BUFFER_SIZE
}

#[unsafe(no_mangle)]
pub extern "C" fn process_bytes(len: usize) -> i32 {
    if len > BUFFER_SIZE {
        return -1;
    }

    unsafe {
	let ptr = SHARED_BUFFER.0.get() as *mut u8;
        let slice = core::slice::from_raw_parts_mut(ptr, len);

        for byte in slice.iter_mut() {
            *byte = !*byte; // Beispiel-Operation (Invertieren)
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn start_bootloader() -> i32 {
  return_encoded(StartBootLoader)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn panic_cmd() -> i32 {
  return_encoded(Panic)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test() -> i32 {
  return_encoded(Test)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn reset() -> i32 {
  return_encoded(Reset)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn led(v: bool) -> i32 {
    return_encoded(LED{ value: v})
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn led_blink(i: u32) -> i32 {
    return_encoded(LEDBlink{ interval: i})
}

/*
#[n(5)]
    LEDBlink{#[n(0)] interval: u32},
    #[n(6)]
    RadioOn{#[n(0)] time: u32},
    #[n(7)]
    RadioOff,
    #[n(8)]
    TxOn{#[n(0)] time: u32},
    #[n(9)]
    TxOff,
    #[n(10)]
    MorseSpeed{#[n(0)] ditlen: u16},
    #[n(11)]
    MorseSend{ #[cbor(n(1), with = "minicbor::bytes")] txt: &'a [u8]},
    #[n(12)]
    MorseAppend{ #[cbor(n(1), with = "minicbor::bytes")] txt: &'a [u8]},
 */


fn return_encoded(cmd: Cmd) -> i32 {
    unsafe {
    let ptr = SHARED_BUFFER.0.get() as *mut u8;
    let slice = core::slice::from_raw_parts_mut(ptr, BUFFER_SIZE);

    let mut cursor = Cursor::new(&mut *slice);

    match minicbor::encode(&cmd, &mut cursor) {
        Ok(_) => {
	    return cursor.position() as i32
	}
	Err(_) => {
	    return -1
	}
    };
    };
}


#[link(wasm_import_module = "env")]
unsafe extern "C" {
    fn host_panic(ptr: *const u8, len: usize);
}


#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // Hole die Nachricht über das moderne info.message() statt .payload()
    let msg_str = info.message().as_str();
    
    // Falls der Text direkt als &str vorliegt, nutzen wir ihn.
    // Falls er komplex formatiert ist (z.B. mit Variablen), nutzen wir einen Fallback,
    // um no_std-Formatierungs-Code-Overhead zu vermeiden.
    let msg = match msg_str {
        Some(s) => s,
        None => "WASM Panic aufgetreten (Details in no_std nicht formatierbar)",
    };

    // Sende Pointer und Länge der Nachricht an JavaScript über den unsafe Block
    unsafe {
        host_panic(msg.as_ptr(), msg.len());
    }
    // Harter Absturz für die WASM-Runtime im Browser
    core::arch::wasm32::unreachable()
}



#[unsafe(no_mangle)]
pub extern "C" fn trigger_panic() {
    panic!("Hier ist deine no_std Fehlermeldung!");
}
