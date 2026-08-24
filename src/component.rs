#[macro_export]
macro_rules! tuple_comp {
    ($ident:ident, $ty:ty, default($default:expr), copy$(, $($tt:tt)*)?) => {
        tuple_comp!($ident, $ty, default($default)$(, $($tt)*)?, derives(Copy));
    };

    ($ident:ident, $ty:ty, default($default:expr), eq$(, $($tt:tt)*)?) => {
        tuple_comp!($ident, $ty, default($default)$(, $($tt)*)?, derives(PartialEq, Eq));
    };

    ($ident:ident, $ty:ty, default($default:expr), ord$(, $($tt:tt)*)?) => {
        tuple_comp!($ident, $ty, default($default)$(, $($tt)*)?,eq, derives(PartialOrd, Ord));
    };

    ($ident:ident, $ty:ty, default($default:expr), hash$(, $($tt:tt)*)?) => {
        tuple_comp!($ident, $ty, default($default)$(, $($tt)*)?, derives(Hash));
    };

    ($ident:ident, $ty:ty, default($default:expr), derives($($derive_a:ident),*), derives($($derive_b:ident),*)$(, $($tt:tt)*)?) => {
        tuple_comp!($ident, $ty, default($default), derives($($derive_a),*, $($derive_b),*));
    };

    ($ident:ident, $ty:ty, default($default:expr), derives($($derive:ident),*)) => {
        #[derive($($derive),*)]
        #[derive(::bevy::prelude::Component)]
        pub struct $ident(pub $ty);
    };

    ($ident:ident, $ty:ty$(, $($tt:tt)*)?) => {
        tuple_comp!($ident, $ty, default(Default::default())$(, $($tt)*)?);
    };
}

#[macro_export]
macro_rules! marker {
    ($ident:ident) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            Default,
            ::bevy::prelude::Component,
        )]
        pub struct $ident;
    };

    ($ident:ident, named) => {
        #[derive(
            Debug,
            Clone,
            Copy,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            Default,
            ::bevy::prelude::Component,
        )]
        #[require(::bevy::prelude::Name(stringify!($ident).into()))]
        pub struct $ident;
    };
}

pub use crate::marker;
pub use crate::tuple_comp;

#[macro_export]
macro_rules! u8_component {
    ($ident:ident$(, $($tt:tt)*)?) => {
        tuple_comp!($ident, u8$(, $($tt)*)?);
    };
}

#[macro_export]
macro_rules! u16_component {
    ($ident:ident$(, $($tt:tt)*)?) => {
        tuple_comp!($ident, u16$(, $($tt)*)?);
    };
}

#[macro_export]
macro_rules! u32_component {
    ($ident:ident$(, $($tt:tt)*)?) => {
        tuple_comp!($ident, u32$(, $($tt)*)?);
    };
}

#[macro_export]
macro_rules! u64_component {
    ($ident:ident$(, $($tt:tt)*)?) => {
        tuple_comp!($ident, u64$(, $($tt)*)?);
    };
}

#[macro_export]
macro_rules! i8_component {
    ($ident:ident$(, $($tt:tt)*)?) => {
        tuple_comp!($ident, i8$(, $($tt)*)?);
    };
}

#[macro_export]
macro_rules! i16_component {
    ($ident:ident$(, $($tt:tt)*)?) => {
        tuple_comp!($ident, i16$(, $($tt)*)?);
    };
}

#[macro_export]
macro_rules! i32_component {
    ($ident:ident$(, $($tt:tt)*)?) => {
        tuple_comp!($ident, i32$(, $($tt)*)?);
    };
}

#[macro_export]
macro_rules! i64_component {
    ($ident:ident$(, $($tt:tt)*)?) => {
        tuple_comp!($ident, i64$(, $($tt)*)?);
    };
}

#[macro_export]
macro_rules! f32_component {
    ($ident:ident$(, $($tt:tt)*)?) => {
        tuple_comp!($ident, f32$(, $($tt)*)?);
    };
}

#[macro_export]
macro_rules! f64_component {
    ($ident:ident$(, $($tt:tt)*)?) => {
        tuple_comp!($ident, f64$(, $($tt)*)?);
    };
}

pub use crate::{
    f32_component, f64_component, i8_component, i16_component, i32_component, i64_component,
    u8_component, u16_component, u32_component, u64_component,
};
