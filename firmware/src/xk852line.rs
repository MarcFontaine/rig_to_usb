use ringbuffer::{ConstGenericRingBuffer, RingBuffer};

use crate::board::Board;

const MAX_LINE_LEN: usize = 32;
const LF: u8 = 10;
const CR: u8 = 13;

type Buffer = ConstGenericRingBuffer::<u8, MAX_LINE_LEN>;

#[derive(Debug)]
pub enum Line {
    Starting,
    NoLF,
    LFSeen,
}
use Line::*;

#[derive(Debug)]
pub enum ParseResult {
    MissingLF,
    MultipleLF,
    Cont,
    FullLine,
}
use ParseResult::*;

fn add_char(msg: &mut Buffer, discarded: &mut Buffer, l: &mut Line, chr: u8) -> ParseResult
{
    match l {
	Starting => if chr == LF
	{
	    msg.enqueue(chr);
	    *l = LFSeen;
	    return Cont;
	}
        else
	{
	    discarded.enqueue(chr);
	    return Cont;
	}
	NoLF => if chr == LF
	{
	    msg.enqueue(chr);
	    *l = LFSeen;
	    return Cont;
	}
        else
	{
	    discarded.enqueue(chr);
	    return MissingLF;
	}
	LFSeen => match chr {
	    LF => {
		discarded.extend(msg.drain());
		msg.enqueue(chr);
		return MultipleLF;
	    }
	    CR => {
                msg.enqueue(chr);
		*l = NoLF;
		return FullLine;
	    }
	    other => {
		msg.enqueue(other);
		return Cont;
	    }
	}
    }
}

#[derive(Debug)]
pub struct LineState {
    l: Line,
    msg: Buffer,
    discarded: Buffer,
    pub errors: Option<Buffer>,
    pub fullmsg: Option<Buffer>,
}


pub fn init_line_state() -> LineState
{
    LineState {
	l:     Starting,
	msg: Buffer::new(),
	discarded: Buffer::new(),
	errors: None,
	fullmsg: None,
    }
}

pub fn update_xk852_line(b: &mut Board, ls: LineState) {
    let mut msg = ls.msg;
    let mut discarded = ls.discarded;
    let mut l = ls.l;
    while let Some(byte) = b.cat_rx.next() {
	let chr = byte & 0x7F;
	defmt::info!("Byte empfangen: {:a}", chr);
        let res = add_char(&mut msg, &mut discarded, &mut l, chr);
	match res {
	    MissingLF => {},
	    MultipleLF => {},
	    Cont => {},
	    FullLine => {},
	}
    }
}
