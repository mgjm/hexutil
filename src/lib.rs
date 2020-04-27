#![warn(missing_docs, clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(clippy::missing_errors_doc)]

//! Implement common traits for binary representable data.
//!
//! Use the `impl_hex` macro to implement the `ToHex`, `FromHex`, `Display`, `FromStr`, `Serialize` and `Deserialize` traits.
//!
//! This can be done by returning a reference to some bytes:
//! ```
//! struct Test([u8; 42]);
//!
//! hexutil::impl_hex!(Test, 42, |&self| &self.0, |data| Ok(Self(data)));
//! ```
//!
//! Or by returning some bytes by value:
//! ```
//! struct Test(u128);
//!
//! hexutil::impl_hex!(Test, 16, |self| self.0.to_le_bytes(), |data| Ok(Self(
//!     u128::from_le_bytes(data)
//! )));
//! ```
//! # Example
//! ```
//! # use hexutil::{ParseHex, ToHex, FromHex};
//! # #[derive(Debug, PartialEq, Eq)]
//! struct Test(u16);
//!
//! hexutil::impl_hex!(Test, 2, |self| self.0.to_le_bytes(), |data| Ok(Self(
//!     u16::from_le_bytes(data)
//! )));
//!
//! let test = Test(0x1234);
//!
//! // std::fmt::Display
//! assert_eq!(format!("{}", test), "3412");
//!
//! // std::string::ToString
//! let hex = test.to_string();
//! assert_eq!(hex, "3412");
//!
//! // std::convert::FromStr
//! let test: Test = hex.parse().unwrap();
//! assert_eq!(test, Test(0x1234));
//!
//! // hexutil::ToHex
//! let hex = test.to_hex();
//! assert_eq!(hex, "3412");
//!
//! // hexutil::FromHex
//! let test = Test::from_hex(hex.as_bytes()).unwrap();
//! assert_eq!(test, Test(0x1234));
//!
//! // hexutil::ParseHex
//! let test: Test = hex.parse_hex().unwrap();
//! assert_eq!(test, Test(0x1234));
//!
//! // serde::Serialize (with serializer.is_human_readable() == true)
//! let json = serde_json::to_string(&test).unwrap();
//! assert_eq!(json, r#""3412""#);
//!
//! // serde::Deserialize (with deserializer.is_human_readable() == true)
//! let test: Test = serde_json::from_str(&json).unwrap();
//! assert_eq!(test, Test(0x1234));
//!
//! // serde::Serialize (with serializer.is_human_readable() == false)
//! let bin = bincode::serialize(&test).unwrap();
//! assert_eq!(bin, [0x34, 0x12]);
//!
//! // serde::Deserialize (with deserializer.is_human_readable() == false)
//! let test: Test = bincode::deserialize(&bin).unwrap();
//! assert_eq!(test, Test(0x1234));
//! ```
//!
//! # Presets
//! You can append a list of presets what do derive:
//!
//! Name | Desciption
//! -|-
//! `default` | `convert` and `serde`
//! `convert` | `Display` and `FromStr`
//! `Display` | Implement the `std::fmt::Display` trait (enables the `to_string()` method)
//! `FromStr` | Implement the `std::convert::FromStr` trait (enables the `str.parse()` method)
//! `serde` | `Serialize` and `Deserialize`
//! `Serialize` | Implement the `serde::Serialize` trait
//! `Deserialize` | Implement the `serde::Deserialize` trait
//!
//! Derive only the `ToHex`, `FromHex`, `Serialize` and `Deserialize` traits:
//! ```
//! struct Test([u8; 42]);
//!
//! hexutil::impl_hex!(Test, 42, |self| self.0, |data| Ok(Self(data)), [serde]);
//! ```
//!
//! # `FromHex` Error
//! The second function returns a `Result<Self, FromHexError>`:
//! ```
//! # use hexutil::FromHexError;
//! struct Test([u8; 42]);
//!
//! hexutil::impl_hex!(Test, 42, |self| self.0, |data| {
//!     Err(FromHexError::CustomStr("can't create this from hex"))
//! });
//! ```
//! Or use `FromHexError::InvalidValue` to display a default message:
//! ```
//! # use hexutil::FromHexError;
//! struct Test([u8; 42]);
//!
//! hexutil::impl_hex!(Test, 42, |self| self.0, |data| Err(
//!     FromHexError::InvalidValue
//! ));
//! ```
//!
//! # One direction only
//! You can also implement only one direction:
//! ```
//! struct Test([u8; 42]);
//!
//! hexutil::impl_to_hex!(Test, 42, |self| self.0);
//! ```
//! ```
//! struct Test([u8; 42]);
//!
//! hexutil::impl_from_hex!(Test, 42, |data| Ok(Self(data)));
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

mod macros;

pub mod unstable;

#[doc(hidden)]
pub mod private {
    pub use serde::{Deserialize, Deserializer, Serialize, Serializer};
}

use err_derive::Error;

#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::string::{String, ToString};

/// A type that can be converted to a hexadecimal representation.
pub trait ToHex: unstable::ToHexCore {
    /// Get a hexadecimal representation.
    ///
    /// ```
    /// # use hexutil::ToHex;
    /// struct Test(u16);
    ///
    /// hexutil::impl_to_hex!(Test, 2, |self| self.0.to_le_bytes());
    ///
    /// assert_eq!(Test(0x1234).to_hex(), "3412");
    /// ```
    #[cfg(feature = "alloc")]
    fn to_hex(&self) -> String where {
        unstable::with_hex_str(self, ToString::to_string)
    }
}

/// A type that can be created from a hexadecimal representation.
pub trait FromHex: unstable::FromHexCore {
    /// Try to create an instance of this type from a hexadeccimal representation.
    ///
    /// ```
    /// # use hexutil::FromHex;
    /// # #[derive(Debug, PartialEq, Eq)]
    /// struct Test(u16);
    ///
    /// hexutil::impl_from_hex!(Test, 2, |data| Ok(Self(u16::from_le_bytes(data))));
    ///
    /// let test = Test::from_hex(b"3412").unwrap();
    /// assert_eq!(test, Test(0x1234));
    /// ```
    fn from_hex(buf: &[u8]) -> Result<Self, FromHexError> {
        if buf.len() & 1 != 0 {
            return Err(FromHexError::InvalidLength(buf.len()));
        }
        let mut bytes = Self::create_bytes(Some(buf.len() / 2));
        {
            let bytes = Self::bytes_as_mut(&mut bytes);
            if bytes.len() * 2 != buf.len() {
                return Err(FromHexError::InvalidLength(buf.len()));
            }
            unstable::decode_hex(buf, bytes)?;
        }
        Self::from_binary_bytes(bytes)
    }
}

/// Parse a hexadecimal value.
///
/// ```
/// # use hexutil::ParseHex;
/// # #[derive(Debug, PartialEq, Eq)]
/// struct Test(u16);
///
/// hexutil::impl_from_hex!(Test, 2, |data| Ok(Self(u16::from_le_bytes(data))));
///
/// let test: Test = "3412".parse_hex().unwrap();
/// assert_eq!(test, Test(0x1234));
/// ```
pub trait ParseHex {
    /// Parse the hexadecimal string representation and create a value of type `T`.
    fn parse_hex<T: FromHex>(self) -> Result<T, FromHexError>;
}

impl ParseHex for &str {
    fn parse_hex<T: FromHex>(self) -> Result<T, FromHexError> {
        self.as_bytes().parse_hex()
    }
}

impl ParseHex for &[u8] {
    fn parse_hex<T: FromHex>(self) -> Result<T, FromHexError> {
        T::from_hex(self)
    }
}

/// An error occured while converting from a hexadecimal value.
#[derive(Debug, Error)]
pub enum FromHexError {
    /// The number of bytes is not valid.
    #[error(display = "invalid number of bytes: {}", 0)]
    InvalidLength(usize),

    /// The carachter at the given index is invalid.
    #[error(display = "invalid hex character at {}: {:?}", 0, char::from(1))]
    InvalidHexCharacter(usize, u8),

    /// The value is not valid.
    #[error(display = "invalid value")]
    InvalidValue,

    /// A custom error (static string reference).
    #[error(display = "{}", 0)]
    CustomStr(&'static str),

    /// A custom error (`String` requires `alloc` or `std` feature).
    #[cfg(feature = "alloc")]
    #[error(display = "{}", 0)]
    CustomString(String),
}
