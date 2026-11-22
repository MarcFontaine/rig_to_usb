use ringbuffer::{ConstGenericRingBuffer, RingBuffer};

use crate::board::Board;
use crate::config;
use crate::config::XK852_MAX_LINE_LEN;

type Buffer = ConstGenericRingBuffer<u8, XK852_MAX_LINE_LEN>;

#[derive(Debug)]
pub enum Line {
    NoLF,
    LFSeen,
}
use Line::*;

fn add_char(msg: &mut Buffer, discarded: &mut Buffer, l: &mut Line, chr: u8) -> Option<Line> {
    match *l {
        NoLF => {
            if chr == config::XK852_LF {
                msg.enqueue(chr);
                return Some(LFSeen);
            } else {
                discarded.enqueue(chr);
                return Some(NoLF);
            }
        }
        LFSeen => match chr {
            config::XK852_LF => {
                discarded.extend(msg.drain());
                msg.enqueue(chr);
                return Some(LFSeen);
            }
            config::XK852_CR => {
                msg.enqueue(chr);
                return None;
            }
            other => {
                msg.enqueue(other);
                return Some(LFSeen);
            }
        },
    }
}

#[derive(Debug)]
pub enum LineState {
    Scanning {
        l: Line,
        msg: Buffer,
        prefix: Buffer,
    },
    ParseResult {
        prefix: Buffer,
        msg: Buffer,
    },
}
use LineState::*;

pub fn init_line_state() -> LineState {
    Scanning {
        l: NoLF,
        prefix: Buffer::new(),
        msg: Buffer::new(),
    }
}

pub fn update_xk852_line(b: &mut Board, ls: &mut LineState) {
    if let Scanning { l, prefix, msg } = ls {
        while let Some(byte) = b.cat_rx.next() {
            let chr = byte & 0x7F;
            defmt::info!("Byte received: {:a}", chr);
            let res = add_char(msg, prefix, l, chr);
            match res {
                Some(next) => {
                    *l = next;
                }
                None => {
                    let mut m = Buffer::new();
                    m.extend(msg.drain());
                    let mut p = Buffer::new();
                    p.extend(prefix.drain());
                    *ls = ParseResult { prefix: p, msg: m };
                    return;
                }
            }
        }
    }
}
