//! Functions required to implement serde traits.

use super::encode_hex;
use crate::{FromHex, FromHexError, ParseHex, ToHex};
use core::{fmt, marker::PhantomData};
use serde::{
    de::{Error, SeqAccess, Unexpected, Visitor},
    ser::SerializeTupleStruct,
    Deserializer, Serializer,
};

/// Metadata required for serialization and deserialization using serde.
pub trait Metadata {
    /// The name of this type.
    const NAME: &'static str;

    /// A string describing the content of this type.
    const EXPECTING: &'static str;

    /// The number of bytes in this type.
    ///
    /// This should be `None` if this type can have a dynamic length.
    const LEN: Option<usize>;
}

/// Serialize a `value` using a `serializer`.
pub fn serialize<S, T>(serializer: S, value: &T) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToHex + Metadata,
{
    let bytes = value.to_binary_bytes();
    let bytes = value.as_binary_bytes(&bytes);
    if serializer.is_human_readable() {
        let mut buffer = T::create_buffer(bytes.len() * 2);
        let buffer = T::buffer_as_bytes(&mut buffer);
        serializer.serialize_str(encode_hex(bytes, buffer))
    } else {
        serialize_bytes(serializer, T::NAME, bytes)
    }
}

fn serialize_bytes<S>(serializer: S, name: &'static str, value: &[u8]) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut serializer = serializer.serialize_tuple_struct(name, value.len())?;
    for item in value {
        serializer.serialize_field(item)?;
    }
    serializer.end()
}

/// Deserialize a `value` using a `deserializer`.
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromHex + Metadata,
{
    if deserializer.is_human_readable() {
        deserializer.deserialize_str(HexVisitor::new())
    } else if let Some(len) = T::LEN {
        deserializer.deserialize_tuple_struct(T::NAME, len, BinaryVisitor::new())
    } else {
        Err(D::Error::custom(format_args!(
            "the type {} does not have a known length",
            T::NAME,
        )))
    }
}

struct HexVisitor<T>
where
    T: FromHex,
{
    seed: PhantomData<T>,
}

impl<T> HexVisitor<T>
where
    T: FromHex,
{
    fn new() -> Self {
        Self { seed: PhantomData }
    }
}

impl<'de, T> Visitor<'de> for HexVisitor<T>
where
    T: FromHex + Metadata,
{
    type Value = T;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(T::EXPECTING)
    }
    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        v.parse_hex()
            .map_err(|err| err.into_serde(Unexpected::Str(v), &self))
    }
}

struct BinaryVisitor<T>
where
    T: FromHex,
{
    seed: PhantomData<T>,
}

impl<T> BinaryVisitor<T>
where
    T: FromHex,
{
    fn new() -> Self {
        Self { seed: PhantomData }
    }
}

impl<'de, T> Visitor<'de> for BinaryVisitor<T>
where
    T: FromHex + Metadata,
{
    type Value = T;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(T::EXPECTING)
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let len = seq.size_hint();
        let mut bytes = T::create_bytes(len);
        let len = {
            let bytes = T::bytes_as_mut(&mut bytes);
            for (i, byte) in bytes.iter_mut().enumerate() {
                if let Some(b) = seq.next_element()? {
                    *byte = b
                } else {
                    return Err(Error::invalid_length(i, &self));
                }
            }
            len.unwrap_or_else(|| bytes.len())
        };
        if seq.next_element::<u8>()?.is_some() {
            Err(Error::invalid_length(len, &self))
        } else {
            T::from_binary_bytes(bytes).map_err(|err| err.into_serde(Unexpected::Seq, &self))
        }
    }
}

impl FromHexError {
    fn into_serde<'de, E: Error, V: Visitor<'de>>(self, unexp: Unexpected, visitor: &V) -> E {
        match self {
            Self::InvalidLength(len) => E::invalid_length(len, visitor),
            Self::InvalidHexCharacter(_, c) => {
                E::invalid_value(Unexpected::Char(c.into()), visitor)
            }
            Self::InvalidValue => E::invalid_value(unexp, visitor),
            Self::CustomStr(msg) => E::invalid_value(unexp, &msg),
            #[cfg(feature = "alloc")]
            Self::CustomString(msg) => E::invalid_value(unexp, &msg.as_ref()),
        }
    }
}
