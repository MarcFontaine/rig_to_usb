use minicbor::{Decode, Encode};

#[cfg(feature = "defmt")]
pub use defmt::{info, warn, error};

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Decode, Encode, Debug)]
#[cbor(map)]
pub struct HIDReport<'a> {
    #[n(0)]
    #[cbor(with = "minicbor::bytes")]
    pub cat_rx_msg : Option <&'a [u8]>,
    #[n(1)]
    #[cbor(with = "minicbor::bytes")]
    pub cat_rx_skipped : Option <&'a [u8]>
}
