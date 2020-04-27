//! Unstable traits and functions.
//!
//! All traits and functions in this module are unstable. They could change in the future.

mod hex;
pub mod serde;

pub use hex::{decode_hex, encode_hex};

use crate::{FromHexError, ToHex};

/// Convert a type to a binary or hexadecimal representation.
pub trait ToHexCore {
    /// The type of the binary representation.
    type Bytes;

    /// The buffer for the hexadecimal representation.
    type Buffer;

    /// Create a new empty buffer of size `len`.
    fn create_buffer(len: usize) -> Self::Buffer;

    /// Return a mutable reference to the bytes in the `buffer`.
    fn buffer_as_bytes(buffer: &mut Self::Buffer) -> &mut [u8];

    /// Create a bytes type.
    fn to_binary_bytes(&self) -> Self::Bytes;

    /// Return a reference to the binary representation.
    fn as_binary_bytes<'a>(&'a self, bytes: &'a Self::Bytes) -> &'a [u8];
}

/// Create a type from a binary or hexadecimal representation.
pub trait FromHexCore: Sized {
    /// The type of the binary representation.
    type Bytes;

    /// Create a bytes type of size `len`.
    fn create_bytes(len: Option<usize>) -> Self::Bytes;

    /// Return a mutable reference to the bytes in `bytes`.
    fn bytes_as_mut(bytes: &mut Self::Bytes) -> &mut [u8];

    /// Create an instance of this type from `bytes`.
    fn from_binary_bytes(bytes: Self::Bytes) -> Result<Self, FromHexError>;
}

/// Get a reference to the hexadecimal representation of a `value`.
pub fn with_hex_str<T, U>(value: &T, f: impl FnOnce(&str) -> U) -> U
where
    T: ?Sized + ToHex,
{
    let bytes = value.to_binary_bytes();
    let bytes = value.as_binary_bytes(&bytes);
    let mut buffer = T::create_buffer(bytes.len() * 2);
    let buffer = T::buffer_as_bytes(&mut buffer);
    f(encode_hex(bytes, buffer))
}
