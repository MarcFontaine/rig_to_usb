use nix::pty::{openpty, OpenptyResult};
use nix::sys::termios::{cfsetspeed, tcgetattr, tcsetattr, BaudRate, SetArg};
use nix::unistd::ttyname;
use std::fs::File;
use std::fs;
use std::os::unix::prelude::OwnedFd;
use std::os::unix::fs::symlink;
use std::os::fd::AsFd;
use std::path::Path;
use std::io::{Read, Write};
use std::io;
use std::fs::OpenOptions;
use std::sync::Arc;
use std::thread;

use rig_to_usb_logic::hid_report::HIDReport;
use rig_to_usb_logic::cmd::Cmd;

const HID_REPORT_SIZE: usize = 64;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync >> {
    let (pty, mut _slave) = create_ptty()?;
    let pty_reader = Arc::new(pty);
    let pty_writer = pty_reader.clone();

    let hid_file = open_hid_device()?;
    let hid_reader = Arc::new(hid_file);
    let hid_writer = hid_reader.clone();

    let hid_to_pty_thread = thread::spawn(move || { hid_to_pty(hid_reader, pty_writer) });
    let pty_to_hid_thread = thread::spawn(move || { pty_to_hid(pty_reader, hid_writer) });
    hid_to_pty_thread.join().unwrap()?;
    pty_to_hid_thread.join().unwrap()?;
    Ok(())

    /*
*/
}

fn create_ptty() -> Result<(File, OwnedFd), io::Error> {
    let pid = std::process::id();

    let symlink_path_str = format!("/tmp/xk852_serial_{}", pid);
    let symlink_path = Path::new(&symlink_path_str);
    let generic_path = Path::new("/tmp/xk852_serial");
    if fs::symlink_metadata(symlink_path).is_ok() {
	let _ = fs::remove_file(symlink_path);
    }
    if fs::symlink_metadata(generic_path).is_ok() {
	let _ = fs::remove_file(generic_path);
    }

    let OpenptyResult { master, slave } = openpty(None, None)?;
    let slave_name = ttyname(&slave)?;
    symlink(&slave_name, symlink_path)?;
    symlink(&slave_name, generic_path)?;
    println!("PTY path: {:?}", slave_name);

    let mut termios = tcgetattr(slave.as_fd())?;

    // 4. Transform configurations into absolute RAW mode
    // Disables echoing, line-buffering, and software flow control (XON/XOFF)
    termios.local_flags.remove(
        nix::sys::termios::LocalFlags::ECHO
        | nix::sys::termios::LocalFlags::ECHOE
        | nix::sys::termios::LocalFlags::ICANON
        | nix::sys::termios::LocalFlags::ISIG
    );
    termios.input_flags.remove(
        nix::sys::termios::InputFlags::IXON
        | nix::sys::termios::InputFlags::IXOFF
        | nix::sys::termios::InputFlags::IXANY
        | nix::sys::termios::InputFlags::ICRNL
    );
    termios.output_flags.remove(nix::sys::termios::OutputFlags::OPOST);

    cfsetspeed(&mut termios, BaudRate::B115200)?;

    tcsetattr(slave.as_fd(), SetArg::TCSANOW, &termios)?;

    // 7. Transition ownership directly into normal Rust I/O files
    let master_file = File::from(master);
    println!("PTY Master successfully bound. Listening for incoming bytes...");
    Ok ((master_file, slave))
}

fn open_hid_device() -> Result<File, io::Error> {
    let hid_path = "/dev/hidraw_RigToUSB";
    let hid_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(hid_path)?;
    Ok (hid_file)
}

fn hid_to_pty(hid_reader: Arc<File>, pty_writer: Arc<File>)
    -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let mut hid_in = hid_reader.as_ref();
    let mut pty_out = pty_writer.as_ref();
    let mut buf = [0u8; HID_REPORT_SIZE];

    loop {
	match hid_in.read(&mut buf) {
	    Ok(0) => break,
	    Ok(n) => {
		let mut decoder = minicbor::Decoder::new(&buf[..n]);
		let hid_report = decoder.decode::<HIDReport>()?;
		if let Some(skipped) = hid_report.cat_rx_skipped {
		    pty_out.write_all(skipped)?;
		};
		if let Some(msg) = hid_report.cat_rx_msg {
		    pty_out.write_all(msg)?;
		};
		let _ = pty_out.flush();
	    }
	    Err(e) => {
		eprintln!("[Error] Reading from HID: {}", e);
		break;
	    }
	}
    }
    Ok(())
}

/*
    println!("[+] Bridge active. Bidirectional piping started...");

    // Thread 1: Client -> PTY -> HID Device
    let t1 = thread::spawn(move || {
        let mut pty_in = pty_reader.as_ref();
        let mut hid_out = hid_writer.as_ref();
        let mut buf = [0u8; HID_REPORT_SIZE];

        loop {
            match pty_in.read(&mut buf) {
                Ok(0) => continue, // Await new client connections
                Ok(n) => {
                    // Send exact byte length or zero-pad up to HID_REPORT_SIZE if required by firmware
                    if let Err(e) = hid_out.write_all(&buf[..n]) {
                        eprintln!("[Error] Writing to HID: {}", e);
                        break;
                    }
                    let _ = hid_out.flush();
                }
                Err(e) => {
                    eprintln!("[Error] Reading from PTY: {}", e);
                    break;
                }
            }
        }
    });

    // Thread 2: HID Device -> PTY -> Client
    let t2 = thread::spawn(move || {
        let mut hid_in = hid_reader.as_ref();
        let mut pty_out = pty_writer.as_ref();
        let mut buf = [0u8; HID_REPORT_SIZE];

        loop {
            match hid_in.read(&mut buf) {
                Ok(0) => break, // HID Device disconnected physically
                Ok(n) => {
                    if let Err(e) = pty_out.write_all(&buf[..n]) {
                        eprintln!("[Error] Writing to PTY client: {}", e);
                        break;
                    }
                    let _ = pty_out.flush();
                }
                Err(e) => {
                    eprintln!("[Error] Reading from HID: {}", e);
                    break;
                }
            }
        }
    });

    // Keep main thread alive
    let _ = t1.join();
    let _ = t2.join();

    Ok(())
}
*/

fn pty_to_hid(pty_reader: Arc<File>, hid_writer: Arc<File>)
    -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    let mut pty = pty_reader.as_ref();
    let mut hid = hid_writer.as_ref();
    let mut buffer = [0u8; 128];
    loop {
        match pty.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected (EOF).");
                break Ok(());
            }
            Ok(bytes_read) => {
                println!("Received: {:?}", &buffer[..bytes_read]);
		let cmd = Cmd::SendCat { bytes: &buffer[..bytes_read] };
		let mut hid_packet = [0u8; 65];
		// Byte 0 MUSS 0 sein für Linux hidraw (wird nicht an das Gerät übertragen)
		hid_packet[0] = 0x00;

                let mut cursor = minicbor::encode::write::Cursor::new(&mut hid_packet[1..]);
                minicbor::encode(&cmd, &mut cursor)?;
                hid.write_all(&hid_packet)?;
            }
            Err(e) => {
                // Check if the error code matches raw POSIX EIO (5)
                if e.raw_os_error() == Some(5) {
                    println!("Client disconnected. Waiting for a new connection connection...");
                    // Optional: Sleep briefly to prevent an aggressive spin-lock loop
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    continue;
                } else {
                    // Return real runtime errors that aren't simple disconnects
                    return Err(Box::new(e));
                }
            }
        }
    }
}
