#[cfg(test)]
use crate::cmd::{message_example};

#[cfg(feature = "std")]
pub fn test()
{
    message_example();
}
