use crate::morse;
use crate::morse::morse_char;

pub fn test() {
    println!("Hello World x");
    morse_char(b'?');
    println!("morse code:  {:?}", morse_char(b'?'));
    morse::test();
}
