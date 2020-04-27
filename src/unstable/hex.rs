#![allow(clippy::module_name_repetitions)]

use crate::FromHexError;

const BYTES: [u8; 16] = *b"0123456789abcdef";

/// Encode a binary `value` into the `buffer` as a hexadecimal representation.
///
/// # Panic
/// Panics if `value.len() * 2 != buffer.len()`.
pub fn encode_hex<'a>(value: &[u8], buffer: &'a mut [u8]) -> &'a str {
    assert_eq!(value.len() * 2, buffer.len());
    value
        .iter()
        .cloned()
        .zip(buffer.chunks_mut(2))
        .for_each(write_hex);
    core::str::from_utf8(buffer).unwrap()
}

fn write_hex((a, b): (u8, &mut [u8])) {
    b[0] = BYTES[usize::from(a >> 4)];
    b[1] = BYTES[usize::from(a & 0xf)];
}

/// Decode a hexadecimal `value` into a binary `buffer`.
pub fn decode_hex<'a>(value: &[u8], buffer: &'a mut [u8]) -> Result<(), FromHexError> {
    if value.len() != buffer.len() * 2 {
        return Err(FromHexError::InvalidLength(value.len()));
    }
    value
        .chunks(2)
        .enumerate()
        .zip(buffer.iter_mut())
        .try_for_each(read_hex)
}

fn read_hex(((i, c), b): ((usize, &[u8]), &mut u8)) -> Result<(), FromHexError> {
    *b = (from_hex(i * 2, c[0])? << 4) | from_hex(i * 2 + 1, c[1])?;
    Ok(())
}

fn from_hex(i: usize, c: u8) -> Result<u8, FromHexError> {
    match c {
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'0'..=b'9' => Ok(c - b'0'),
        _ => Err(FromHexError::InvalidHexCharacter(i, c)),
    }
}
