#[doc(hidden)]
#[macro_export]
macro_rules! private_impl_to_hex {
    ($ty:ident, $len:expr, |$self:ident| $to_hex:expr) => {
        #[doc(hidden)]
        impl $crate::unstable::ToHexCore for $ty {
            type Bytes = [u8; $len];
            type Buffer = [u8; $len * 2];
            fn create_buffer(_len: usize) -> Self::Buffer {
                [0; $len * 2]
            }
            fn buffer_as_bytes(buffer: &mut Self::Buffer) -> &mut [u8] {
                buffer
            }
            fn to_binary_bytes(&$self) -> Self::Bytes {
                $to_hex
            }
            fn as_binary_bytes<'a>(&'a $self, bytes: &'a Self::Bytes) -> &'a [u8] {
                bytes
            }
        }
        impl $crate::ToHex for $ty {}
    };
    ($ty:ident, $len:expr, |&$self:ident| $to_hex:expr) => {
        #[doc(hidden)]
        impl $crate::unstable::ToHexCore for $ty {
            type Bytes = ();
            type Buffer = [u8; $len * 2];
            fn create_buffer(_len: usize) -> Self::Buffer {
                [0; $len * 2]
            }
            fn buffer_as_bytes(buffer: &mut Self::Buffer) -> &mut [u8] {
                buffer
            }
            fn to_binary_bytes(&self) -> Self::Bytes {
                ()
            }
            fn as_binary_bytes(&$self, _bytes: &Self::Bytes) -> &[u8] {
                $to_hex
            }
        }
        impl $crate::ToHex for $ty {}
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! private_to_hex_preset {
    ($ty:ident, default) => {
        $crate::private_to_hex_preset!($ty, convert);
        $crate::private_to_hex_preset!($ty, serde);
    };
    ($ty:ident, convert) => {
        $crate::private_to_hex_preset!($ty, Display);
    };
    ($ty:ident, Display) => {
        impl ::core::fmt::Display for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                $crate::unstable::with_hex_str(self, |s| f.write_str(s))
            }
        }
    };
    ($ty:ident, serde) => {
        $crate::private_to_hex_preset!($ty, Serialize);
    };
    ($ty:ident, Serialize) => {
        impl $crate::private::Serialize for $ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::private::Serializer,
            {
                $crate::unstable::serde::serialize(serializer, self)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! private_to_hex_presets {
    ($ty:ident,) => {};
    ($ty:ident, $preset:ident, $($presets:ident,)*) => {
        $crate::private_to_hex_preset!($ty, $preset);
        $crate::private_to_hex_presets!($ty, $($presets,)*);
    };
}

/// Implement common traits for binary representable data (to hex only).
///
/// See [crate-level documentation](index.html) for more information.
#[macro_export]
macro_rules! impl_to_hex {
    ($ty:ident, $len:expr, |$self:ident| $to_hex:expr $(,)?) => {
        $crate::impl_to_hex!($ty, $len, |$self| $to_hex, [default]);
    };
    ($ty:ident, $len:expr, |&$self:ident| $to_hex:expr $(,)?) => {
        $crate::impl_to_hex!($ty, $len, |&$self| $to_hex, [default]);
    };
    ($ty:ident, $len:expr, |$self:ident| $to_hex:expr, [$($preset:ident),* $(,)?] $(,)?) => {
        $crate::private_impl_to_hex!($ty, $len, |$self| $to_hex);
        $crate::private_metadata!($ty, $len, $($preset,)*);
        $crate::private_to_hex_presets!($ty, $($preset,)*);
    };
    ($ty:ident, $len:expr, |&$self:ident| $to_hex:expr, [$($preset:ident),* $(,)?] $(,)?) => {
        $crate::private_impl_to_hex!($ty, $len, |&$self| $to_hex);
        $crate::private_metadata!($ty, $len, $($preset,)*);
        $crate::private_to_hex_presets!($ty, $($preset,)*);
    };
}
