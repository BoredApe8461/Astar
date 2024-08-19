#![cfg_attr(not(feature = "std"), no_std)]
use parity_scale_codec::MaxEncodedLen;
use parity_scale_codec::{Decode, Encode};
use sp_runtime::{traits::Printable, DispatchError, DispatchErrorWithPostInfo, ModuleError};

#[derive(PartialEq, Eq, Copy, Clone, Encode, Decode, Debug)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Outcome {
    /// Success
    Success = 0,
    /// The signing account has no permission to do the operation.
    NoPermission = 1,
    /// The given item ID is unknown.
    UnknownCollection = 2,
    /// The item ID has already been used for an item.
    AlreadyExists = 3,
    /// The owner turned out to be different to what was expected.
    WrongOwner = 4,
    /// Invalid witness data given.
    BadWitness = 5,
    /// The item ID is already taken.
    InUse = 6,
    /// The item or collection is frozen.
    Frozen = 7,
    /// The delegate turned out to be different to what was expected.
    WrongDelegate = 8,
    /// There is no delegate approved.
    NoDelegate = 9,
    /// No approval exists that would allow the transfer.
    Unapproved = 10,
    /// The named owner has not signed ownership of the collection is acceptable.
    Unaccepted = 11,
    /// The item is locked.
    Locked = 12,
    /// All items have been minted.
    MaxSupplyReached = 13,
    /// The max supply has already been set.
    MaxSupplyAlreadySet = 14,
    /// The provided max supply is less than the amount of items a collection already has.
    MaxSupplyTooSmall = 15,
    /// The given item ID is unknown.
    UnknownItem = 16,
    /// Item is not for sale.
    NotForSale = 17,
    /// The provided bid is too low.
    BidTooLow = 18,
    /// Origin Caller is not supported
    OriginCannotBeCaller = 98,
    /// Unknown error
    RuntimeError = 99,
}


impl From<DispatchError> for Outcome {
    fn from(input: DispatchError) -> Self {
        let error_text = match input {
            DispatchError::Module(ModuleError { message, .. }) => message,
            _ => Some("No module error Info"),
        };
        return match error_text {
            Some("NoPermission") => Outcome::NoPermission,
            Some("UnknownCollection") => Outcome::UnknownCollection,
            Some("AlreadyExists") => Outcome::AlreadyExists,
            Some("WrongOwner") => Outcome::WrongOwner,
            Some("BadWitness") => Outcome::BadWitness,
            Some("InUse") => Outcome::InUse,
            Some("Frozen") => Outcome::Frozen,
            Some("WrongDelegate") => Outcome::WrongDelegate,
            Some("NoDelegate") => Outcome::NoDelegate,
            Some("Unapproved") => Outcome::Unapproved,
            Some("Unaccepted") => Outcome::Unaccepted,
            Some("Locked") => Outcome::Locked,
            Some("MaxSupplyReached") => Outcome::MaxSupplyReached,
            Some("MaxSupplyAlreadySet") => Outcome::MaxSupplyAlreadySet,
            Some("MaxSupplyTooSmall") => Outcome::MaxSupplyTooSmall,
            Some("UnknownItem") => Outcome::UnknownItem,
            Some("NotForSale") => Outcome::NotForSale,
            Some("BidTooLow") => Outcome::BidTooLow,
            _ => Outcome::RuntimeError,
        };
    }
}

impl<Info: Eq + PartialEq + Clone + Copy + Encode + Decode + Printable>
    From<DispatchErrorWithPostInfo<Info>> for Outcome
{
    fn from(input: DispatchErrorWithPostInfo<Info>) -> Self {
        input.error.into()
    }
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
