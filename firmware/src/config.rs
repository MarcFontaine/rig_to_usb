use git_version::git_version;

pub const VERSION: &str = git_version!(prefix = "git:", cargo_prefix = "cargo:", fallback = "unknown");

pub const USB_VID: u16 = 0x1209;
pub const USB_PID: u16 = 0x0001;
pub const USB_MANUFACTURER: &str = "DM1MF";
pub const USB_PRODUCT: &str = "RigToUSB";
pub const USB_SERIAL_NUMBER: &str = VERSION;

pub const XK852_MAX_LINE_LEN: usize = 34;
pub const XK852_LF: u8 = 10;
pub const XK852_CR: u8 = 13;
//pub const XK852_LF: u8 = '(' as u8;
//pub const XK852_CR: u8 = ')' as u8;
