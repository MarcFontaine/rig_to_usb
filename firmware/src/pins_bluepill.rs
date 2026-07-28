use crate::hal::{*};

pub type LED = Pin<'C',13, Output<PushPull>>;
pub type Hochschalten = Pin<'C',14, Output<PushPull>>;
pub type OnOff = Pin<'C',15, Output<PushPull>>;
