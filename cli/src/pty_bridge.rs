use nix::pty::{OpenptyResult, openpty};
use nix::sys::termios::{BaudRate, SetArg, cfsetspeed, tcgetattr, tcsetattr};
use nix::unistd::ttyname;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::os::unix::fs::symlink;
use std::os::unix::prelude::OwnedFd;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

use clap::{ArgAction, Parser};
use git_version::git_version;

use rig_to_usb_logic::cmd::Cmd;
use rig_to_usb_logic::hid_report::HIDReport;

const HID_REPORT_SIZE: usize = 64;

const VERSION: &str = git_version!(
    prefix = concat!(env!("CARGO_PKG_VERSION"), " ("),
    suffix = ")"
);

/// Create a pty from a rigToUSB HID device
#[derive(Parser)]
#[command(
    name = env!("CARGO_BIN_NAME"),
    version = VERSION,
    about
)]
struct Args {
    /// HID device path
    #[arg(long, default_value = "/dev/hidraw_RigToUSB", value_parser = validate_hid_device)]
    hid: PathBuf,
    /// generic pty device path (symlink)
    #[arg(long, default_value = "/tmp/xk852_serial")]
    pty: PathBuf,
    /// Debug-Level (-d, -dd, -ddd)
    #[arg(short, long, action = ArgAction::Count)]
    debug: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Args::parse();
    let (pty, mut _slave) = create_ptty(args.pty)?;
    let pty_reader = Arc::new(pty);
    let pty_writer = pty_reader.clone();

    let hid_file = open_hid_device(args.hid)?;
    let hid_reader = Arc::new(hid_file);
    let hid_writer = hid_reader.clone();

    let hid_to_pty_thread = thread::spawn(move || hid_to_pty(hid_reader, pty_writer));
    let pty_to_hid_thread = thread::spawn(move || pty_to_hid(pty_reader, hid_writer, args.debug));
    hid_to_pty_thread.join().unwrap()?;
    pty_to_hid_thread.join().unwrap()?;
    Ok(())

    /*
     */
}

fn create_ptty(generic_path: PathBuf) -> Result<(File, OwnedFd), io::Error> {
    let symlink_path = PathBuf::from(format!(
        "{}_{}",
        generic_path.display(),
        std::process::id()
    ));

    if fs::symlink_metadata(&symlink_path).is_ok() {
        let _ = fs::remove_file(&symlink_path);
    }
    if fs::symlink_metadata(&generic_path).is_ok() {
        let _ = fs::remove_file(&generic_path);
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
            | nix::sys::termios::LocalFlags::ISIG,
    );
    termios.input_flags.remove(
        nix::sys::termios::InputFlags::IXON
            | nix::sys::termios::InputFlags::IXOFF
            | nix::sys::termios::InputFlags::IXANY
            | nix::sys::termios::InputFlags::ICRNL,
    );
    termios
        .output_flags
        .remove(nix::sys::termios::OutputFlags::OPOST);

    cfsetspeed(&mut termios, BaudRate::B115200)?;

    tcsetattr(slave.as_fd(), SetArg::TCSANOW, &termios)?;

    // 7. Transition ownership directly into normal Rust I/O files
    let master_file = File::from(master);
    println!("PTY Master successfully bound. Listening for incoming bytes...");
    Ok((master_file, slave))
}

fn open_hid_device(hid_path: PathBuf) -> Result<File, io::Error> {
    let hid_file = OpenOptions::new().read(true).write(true).open(hid_path)?;
    Ok(hid_file)
}

fn hid_to_pty(
    hid_reader: Arc<File>,
    pty_writer: Arc<File>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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

fn pty_to_hid(
    pty_reader: Arc<File>,
    hid_writer: Arc<File>,
    debug_level: u8,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
                if debug_level > 0 {
                    println!("Received: {:?}", &buffer[..bytes_read]);
                }
                let cmd = Cmd::SendCat {
                    bytes: &buffer[..bytes_read],
                };
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

fn validate_hid_device(s: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(s);

    if !path.exists() {
        return Err(format!("'{}' : Device not found.", s));
    }

    OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)
        .map_err(|e| format!("'{} : Permissions denied : {}", s, e))?;

    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_cli() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
}
