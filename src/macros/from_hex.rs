#[doc(hidden)]
#[macro_export]
macro_rules! private_impl_from_hex {
    ($ty:ident, $len:expr, |$data:ident| $from_hex:expr) => {
        #[doc(hidden)]
        impl $crate::unstable::FromHexCore for $ty {
            type Bytes = [u8; $len];
            fn create_bytes(_len: Option<usize>) -> Self::Bytes {
                [0; $len]
            }
            fn bytes_as_mut(bytes: &mut Self::Bytes) -> &mut [u8] {
                bytes
            }
            fn from_binary_bytes($data: Self::Bytes) -> Result<Self, $crate::FromHexError> {
                $from_hex
            }
        }
        impl $crate::FromHex for $ty {}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! private_from_hex_preset {
    ($ty:ident, default) => {
        $crate::private_from_hex_preset!($ty, convert);
        $crate::private_from_hex_preset!($ty, serde);
    };
    ($ty:ident, convert) => {
        $crate::private_from_hex_preset!($ty, FromStr);
    };
    ($ty:ident, FromStr) => {
        impl ::core::str::FromStr for $ty {
            type Err = $crate::FromHexError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                $crate::ParseHex::parse_hex(s)
            }
        }
    };
    ($ty:ident, serde) => {
        $crate::private_from_hex_preset!($ty, Deserialize);
    };
    ($ty:ident, Deserialize) => {
        impl<'de> $crate::private::Deserialize<'de> for $ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: $crate::private::Deserializer<'de>,
            {
                $crate::unstable::serde::deserialize(deserializer)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! private_from_hex_presets {
    ($ty:ident,) => {};
    ($ty:ident, $preset:ident, $($presets:ident,)*) => {
        $crate::private_from_hex_preset!($ty, $preset);
        $crate::private_from_hex_presets!($ty, $($presets,)*);
    };
}

/// Implement common traits for binary representable data (from hex only).
///
/// See [crate-level documentation](index.html) for more information.
#[macro_export]
macro_rules! impl_from_hex {
    ($ty:ident, $len:expr, |$data:ident| $from_hex:expr $(,)?) => {
        $crate::impl_from_hex!($ty, $len, |$data| $from_hex, [default]);
    };
    ($ty:ident, $len:expr, |$data:ident| $from_hex:expr, [$($preset:ident),* $(,)?] $(,)?) => {
        $crate::private_impl_from_hex!($ty, $len, |$data| $from_hex);
        $crate::private_metadata!($ty, $len, $($preset,)*);
        $crate::private_from_hex_presets!($ty, $($preset,)*);
    };
}
