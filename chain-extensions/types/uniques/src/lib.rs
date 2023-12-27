#![cfg_attr(not(feature = "std"), no_std)]
use parity_scale_codec::MaxEncodedLen;
use parity_scale_codec::{Decode, Encode};

#[derive(PartialEq, Eq, Copy, Clone, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Outcome {
    /// Success
    Success = 0,
    /// Origin Caller is not supported
    OriginCannotBeCaller = 98,
    /// Unknown error
    RuntimeError = 99,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Encode, Decode, MaxEncodedLen)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Origin {
    Caller,
    Address,
}

impl Default for Origin {
    fn default() -> Self {
        Self::Address
    }
}

#[macro_export]
macro_rules! select_origin {
    ($origin:expr, $account:expr) => {
        match $origin {
            Origin::Caller => return Ok(RetVal::Converging(Outcome::OriginCannotBeCaller as u32)),
            Origin::Address => RawOrigin::Signed($account),
        }
    };
}
