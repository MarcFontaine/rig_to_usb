#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub enum PinState {    
    High,
    Low,
}
use PinState::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Symbol {    
    Dit,
    Dah,
}

use Symbol::*;

pub fn morse_char(c: u8) -> &'static [Symbol] {
    match c.to_ascii_lowercase() {
        // Buchstaben
        b'a' => &[Dit, Dah],
        b'b' => &[Dah, Dit, Dit, Dit],
        b'c' => &[Dah, Dit, Dah, Dit],
        b'd' => &[Dah, Dit, Dit],
        b'e' => &[Dit],
        b'f' => &[Dit, Dit, Dah, Dit],
        b'g' => &[Dah, Dah, Dit],
        b'h' => &[Dit, Dit, Dit, Dit],
        b'i' => &[Dit, Dit],
        b'j' => &[Dit, Dah, Dah, Dah],
        b'k' => &[Dah, Dit, Dah],
        b'l' => &[Dit, Dah, Dit, Dit],
        b'm' => &[Dah, Dah],
        b'n' => &[Dah, Dit],
        b'o' => &[Dah, Dah, Dah],
        b'p' => &[Dit, Dah, Dah, Dit],
        b'q' => &[Dah, Dah, Dit, Dah],
        b'r' => &[Dit, Dah, Dit],
        b's' => &[Dit, Dit, Dit],
        b't' => &[Dah],
        b'u' => &[Dit, Dit, Dah],
        b'v' => &[Dit, Dit, Dit, Dah],
        b'w' => &[Dit, Dah, Dah],
        b'x' => &[Dah, Dit, Dit, Dah],
        b'y' => &[Dah, Dit, Dah, Dah],
        b'z' => &[Dah, Dah, Dit, Dit],

        // Zahlen
        b'1' => &[Dit, Dah, Dah, Dah, Dah],
        b'2' => &[Dit, Dit, Dah, Dah, Dah],
        b'3' => &[Dit, Dit, Dit, Dah, Dah],
        b'4' => &[Dit, Dit, Dit, Dit, Dah],
        b'5' => &[Dit, Dit, Dit, Dit, Dit],
        b'6' => &[Dah, Dit, Dit, Dit, Dit],
        b'7' => &[Dah, Dah, Dit, Dit, Dit],
        b'8' => &[Dah, Dah, Dah, Dit, Dit],
        b'9' => &[Dah, Dah, Dah, Dah, Dit],
        b'0' => &[Dah, Dah, Dah, Dah, Dah],

        // Gängige Sonderzeichen
        b'.' => &[Dit, Dah, Dit, Dah, Dit, Dah],         // Punkt
        b',' => &[Dah, Dah, Dit, Dit, Dah, Dah],         // Komma
        b'?' => &[Dit, Dit, Dah, Dah, Dit, Dit],         // Fragezeichen
        b'\'' => &[Dit, Dah, Dah, Dah, Dah, Dit],        // Apostroph
        b'!' => &[Dah, Dit, Dah, Dit, Dah, Dah],         // Ausrufezeichen
        b'/' => &[Dah, Dit, Dit, Dah, Dit],              // Schrägstrich
        b'(' => &[Dah, Dit, Dah, Dah, Dit],              // Klammer auf
        b')' => &[Dah, Dit, Dah, Dah, Dit, Dah],         // Klammer zu
        b'&' => &[Dit, Dah, Dit, Dit, Dit],              // Und-Zeichen
        b':' => &[Dah, Dah, Dah, Dit, Dit, Dit],         // Doppelpunkt
        b';' => &[Dah, Dit, Dah, Dit, Dah, Dit],         // Semikolon
        b'=' => &[Dah, Dit, Dit, Dit, Dah],              // Gleichheitszeichen
        b'+' => &[Dit, Dah, Dit, Dah, Dit],              // Plus
        b'-' => &[Dah, Dit, Dit, Dit, Dit, Dah],         // Bindestrich/Minus
        b'_' => &[Dit, Dit, Dah, Dah, Dit, Dah],         // Unterstrich
        b'"' => &[Dit, Dah, Dit, Dit, Dah, Dit],         // Anführungszeichen
        b'$' => &[Dit, Dit, Dit, Dah, Dit, Dit, Dah],    // Dollar
        b'@' => &[Dit, Dah, Dah, Dit, Dah, Dit],         // At-Zeichen
        // Unbekannte Zeichen / Leerzeichen
        _ => &[],
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharState {
    S(&'static [Symbol]),
    H(u8, &'static [Symbol]),
    L(u8, &'static [Symbol]),
}
use CharState::*;

// pause_len must be >= 1
pub fn next_pin_state_char(pause_len: u8, s: CharState) -> Option<(PinState, CharState)>
{
    match s {
	S([]) => None,
	S([Dit, rest @ ..]) => Some((High, H(1,rest))),
	S([Dah, rest @ ..]) => Some((High, H(3,rest))),
	L(1,[]) => None,
	L(1,[Dit, rest @ ..]) => Some((High, H(1,rest))),
	L(1,[Dah, rest @ ..]) => Some((High, H(3,rest))),
	L(n,s) => Some((Low, L(n-1, s))),
	H(1,[]) => Some((Low, L(pause_len, &[]))),
	H(1,s) => Some((Low, L(1, s))),
	H(n,s) => Some((High, H(n-1, s))),
    }
}

pub fn test()
{
    print!("->");
    let mut s = S( &[Dit, Dit, Dah, Dah, Dit, Dit] );
    loop {
	match next_pin_state_char(10, s) {
	    None => { break;}
	    Some((High, n)) => {
		print!("#");
		s = n;
	    }
	    Some((Low, n)) => {
		print!(" ");
		s = n;
	    }

	}
    };
    println!("<-");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineState {
    state: CharState,
    rest_line: &'static [u8]
}

pub fn next_pin_state(ls: LineState) -> Option<(PinState, LineState)>
{
    match ( ls.rest_line, next_pin_state_char(3, ls.state) ) {
	(_,  Some((level, next))) => Some((level, LineState { state: next, rest_line: ls.rest_line})),
	([], None) => None, // todo: fix add end of word pause
	([ c, r @ ..], None) => next_pin_state( LineState { state: S(morse_char(*c)), rest_line: r}),
    }
}


pub fn test2()
{
    print!("->");
    let mut s = LineState {
	state: S(&[]),
	rest_line: b"Hello"
    };

    loop {
	match next_pin_state(s) {
	    None => { break;}
	    Some((High, n)) => {
		print!("#");
		s = n;
	    }
	    Some((Low, n)) => {
		print!(" ");
		s = n;
	    }

	}
    };
    println!("<-");
}
