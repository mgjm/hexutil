#[doc(hidden)]
#[macro_export]
macro_rules! private_hex_preset {
    ($ty:ident, default) => {
        $crate::private_hex_preset!($ty, convert);
        $crate::private_hex_preset!($ty, serde);
    };
    ($ty:ident, convert) => {
        $crate::private_hex_preset!($ty, Display);
        $crate::private_hex_preset!($ty, FromStr);
    };
    ($ty:ident, Display) => {
        $crate::private_to_hex_preset!($ty, Display);
    };
    ($ty:ident, FromStr) => {
        $crate::private_from_hex_preset!($ty, FromStr);
    };
    ($ty:ident, serde) => {
        $crate::private_hex_preset!($ty, Serialize);
        $crate::private_hex_preset!($ty, Deserialize);
    };
    ($ty:ident, Serialize) => {
        $crate::private_to_hex_preset!($ty, Serialize);
    };
    ($ty:ident, Deserialize) => {
        $crate::private_from_hex_preset!($ty, Deserialize);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! private_hex_presets {
    ($ty:ident,) => {};
    ($ty:ident, $preset:ident, $($presets:ident,)*) => {
        $crate::private_hex_preset!($ty, $preset);
        $crate::private_hex_presets!($ty, $($presets,)*);
    };
}

/// Implement common traits for binary representable data.
///
/// See [crate-level documentation](index.html) for more information.
#[macro_export]
macro_rules! impl_hex {
    ($ty:ident, $len:expr, |$self:ident| $to_hex:expr, |$data:ident| $from_hex:expr $(,)?) => {
        $crate::impl_hex!($ty, $len, |$self| $to_hex, |$data| $from_hex, [default]);
    };
    ($ty:ident, $len:expr, |&$self:ident| $to_hex:expr, |$data:ident| $from_hex:expr $(,)?) => {
        $crate::impl_hex!($ty, $len, |&$self| $to_hex, |$data| $from_hex, [default]);
    };
    ($ty:ident, $len:expr, |$self:ident| $to_hex:expr, |$data:ident| $from_hex:expr, [$($preset:ident),* $(,)?] $(,)?) => {
        $crate::private_impl_to_hex!($ty, $len, |$self| $to_hex);
        $crate::private_impl_from_hex!($ty, $len, |$data| $from_hex);
        $crate::private_metadata!($ty, $len, $($preset,)*);
        $crate::private_hex_presets!($ty, $($preset,)*);
    };
    ($ty:ident, $len:expr, |&$self:ident| $to_hex:expr, |$data:ident| $from_hex:expr, [$($preset:ident),* $(,)?] $(,)?) => {
        $crate::private_impl_to_hex!($ty, $len, |&$self| $to_hex);
        $crate::private_impl_from_hex!($ty, $len, |$data| $from_hex);
        $crate::private_metadata!($ty, $len, $($preset,)*);
        $crate::private_hex_presets!($ty, $($preset,)*);
    };
}
