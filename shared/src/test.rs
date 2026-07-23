use crate::morse::morse_char;
use crate::morse;
use crate::cmd::cbor_message_test;

pub fn test()
{
    println!("Hello World x");
    cbor_message_test();
    morse_char(b'?');
    println!("morse code:  {:?}", morse_char(b'?') );
    morse::test();
}
