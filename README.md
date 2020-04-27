[![Crates.io](https://img.shields.io/crates/v/hexutil.svg)](https://crates.io/crates/hexutil)
[![API Documentation](https://docs.rs/hexutil/badge.svg)](https://docs.rs/hexutil)
[![Workflow Status](https://github.com/mgjm/hexutil/workflows/build/badge.svg)](https://github.com/mgjm/hexutil/actions?query=workflow%3A%22build%22)

# hexutil

Implement common traits for binary representable data.

Use the `impl_hex` macro to implement the `ToHex`, `FromHex`, `Display`, `FromStr`, `Serialize` and `Deserialize` traits.

This can be done by returning a reference to some bytes:
```rust
struct Test([u8; 42]);

hexutil::impl_hex!(Test, 42, |&self| &self.0, |data| Ok(Self(data)));
```

Or by returning some bytes by value:
```rust
struct Test(u128);

hexutil::impl_hex!(Test, 16, |self| self.0.to_le_bytes(), |data| Ok(Self(
    u128::from_le_bytes(data)
)));
```
## Example
```rust
struct Test(u16);

hexutil::impl_hex!(Test, 2, |self| self.0.to_le_bytes(), |data| Ok(Self(
    u16::from_le_bytes(data)
)));

let test = Test(0x1234);

// std::fmt::Display
assert_eq!(format!("{}", test), "3412");

// std::string::ToString
let hex = test.to_string();
assert_eq!(hex, "3412");

// std::convert::FromStr
let test: Test = hex.parse().unwrap();
assert_eq!(test, Test(0x1234));

// hexutil::ToHex
let hex = test.to_hex();
assert_eq!(hex, "3412");

// hexutil::FromHex
let test = Test::from_hex(hex.as_bytes()).unwrap();
assert_eq!(test, Test(0x1234));

// hexutil::ParseHex
let test: Test = hex.parse_hex().unwrap();
assert_eq!(test, Test(0x1234));

// serde::Serialize (with serializer.is_human_readable() == true)
let json = serde_json::to_string(&test).unwrap();
assert_eq!(json, r#""3412""#);

// serde::Deserialize (with deserializer.is_human_readable() == true)
let test: Test = serde_json::from_str(&json).unwrap();
assert_eq!(test, Test(0x1234));

// serde::Serialize (with serializer.is_human_readable() == false)
let bin = bincode::serialize(&test).unwrap();
assert_eq!(bin, [0x34, 0x12]);

// serde::Deserialize (with deserializer.is_human_readable() == false)
let test: Test = bincode::deserialize(&bin).unwrap();
assert_eq!(test, Test(0x1234));
```

## Presets
You can append a list of presets what do derive:

Name | Desciption
-|-
`default` | `convert` and `serde`
`convert` | `Display` and `FromStr`
`Display` | Implement the `std::fmt::Display` trait (enables the `to_string()` method)
`FromStr` | Implement the `std::convert::FromStr` trait (enables the `str.parse()` method)
`serde` | `Serialize` and `Deserialize`
`Serialize` | Implement the `serde::Serialize` trait
`Deserialize` | Implement the `serde::Deserialize` trait

Derive only the `ToHex`, `FromHex`, `Serialize` and `Deserialize` traits:
```rust
struct Test([u8; 42]);

hexutil::impl_hex!(Test, 42, |self| self.0, |data| Ok(Self(data)), [serde]);
```

## `FromHex` Error
The second function returns a `Result<Self, FromHexError>`:
```rust
struct Test([u8; 42]);

hexutil::impl_hex!(Test, 42, |self| self.0, |data| {
    Err(FromHexError::CustomStr("can't create this from hex"))
});
```
Or use `FromHexError::InvalidValue` to display a default message:
```rust
struct Test([u8; 42]);

hexutil::impl_hex!(Test, 42, |self| self.0, |data| Err(
    FromHexError::InvalidValue
));
```

## One direction only
You can also implement only one direction:
```rust
struct Test([u8; 42]);

hexutil::impl_to_hex!(Test, 42, |self| self.0);
```
```rust
struct Test([u8; 42]);

hexutil::impl_from_hex!(Test, 42, |data| Ok(Self(data)));
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
