#[doc(hidden)]
#[macro_export]
macro_rules! private_metadata {
    ($ty:ident, $len:expr,) => {};
    ($ty:ident, $len:expr, default, $($presets:ident,)*) => {
        $crate::private_metadata!($ty, $len, serde,);
    };
    ($ty:ident, $len:expr, serde, $($presets:ident,)*) => {
        #[doc(hidden)]
        impl $crate::unstable::serde::Metadata for $ty {
            const NAME: &'static str = stringify!($ty);
            const EXPECTING: &'static str = concat!("a valid ", stringify!($ty), " (", stringify!($len), " bytes of data)");
            const LEN: Option<usize> = Some($len);
        }
    };
    ($ty:ident, $len:expr, Serialize, $($presets:ident,)*) => {
        $crate::private_metadata!($ty, $len, serde,);
    };
    ($ty:ident, $len:expr, Deserialize, $($presets:ident,)*) => {
        $crate::private_metadata!($ty, $len, serde,);
    };
    ($ty:ident, $len:expr, $preset:ident, $($presets:ident,)*) => {
        $crate::private_metadata!($ty, $len, $($presets,)*);
    };
}
