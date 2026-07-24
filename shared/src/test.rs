use crate::morse::morse_char;
use crate::morse;

pub fn test()
{
    println!("Hello World x");
    morse_char(b'?');
    println!("morse code:  {:?}", morse_char(b'?') );
    morse::test();
}
