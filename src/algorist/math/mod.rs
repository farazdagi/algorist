//! # Mathematical utilities.
//!
//! Various mathematical utilities, including number theory, modular arithmetic,
//! and more.
//!
//! # Number theory
//!
//! For working with prime numbers, see the functions in [`primes`] module.

pub mod gcd;
pub mod log;
pub mod modulo;
pub mod primes;
pub mod root;

pub use gcd::{gcd, gcd_extended, lcm};
use {
    core::fmt::Display,
    std::{
        convert::From,
        fmt::Debug,
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
        str::FromStr,
    },
};

/// Wrapper for a value of type `T`.
pub trait Value<T>: Copy + Clone + Eq + Ord + Default {
    fn val() -> T;
}

/// Type has a constant value of type `T`.
pub trait ConstValue<T>: Value<T> {
    const VAL: T;
}

impl<T, V: ConstValue<T>> Value<T> for V {
    fn val() -> T {
        V::VAL
    }
}

#[macro_export]
macro_rules! value_impl {
    ($name:ident : $t:ty = $val:expr) => {
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
        pub struct $name {}

        impl $crate::math::ConstValue<$t> for $name {
            const VAL: $t = $val;
        }
    };
}
pub use value_impl as value;

/// Type has a zero value.
pub trait Zero {
    fn zero() -> Self;
}

/// Type has a unit value.
pub trait One {
    fn one() -> Self;
}

#[macro_export]
macro_rules! zero_impl {
    ($($t: ident)+) => {$(
        impl $crate::math::Zero for $t {
            fn zero() -> Self {
                0
            }
        }
    )+};
}

zero_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

#[macro_export]
macro_rules! one_impl {
    ($($t: ident)+) => {$(
        impl $crate::math::One for $t {
            fn one() -> Self {
                1
            }
        }
    )+};
}

one_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

/// Trait for down-casting numeric types.
///
/// Potential loss of precision may occur when down-casting from a wider type to
/// a narrower type.
pub trait Downcast: Sized {
    type Source: From<Self>;

    fn downcast(src: Self::Source) -> Self;
}

#[macro_export]
macro_rules! downcast_impl {
    ($($t: ident $w: ident),+) => {$(
        impl $crate::math::Downcast for $t {
            type Source = $w;
            fn downcast(src: Self::Source) -> Self {
                src as $t
            }
        }
    )+};
}
downcast_impl!(i8 i16, i16 i32, i32 i64, i64 i128, u8 u16, u16 u32, u32 u64, u64 u128);

/// Trait for up-casting numeric types.
pub trait Upcast: Sized {
    type Target: From<Self>;

    fn upcast(self) -> Self::Target;
}

#[macro_export]
macro_rules! upcast_impl {
    ($($t: ident $w: ident),+) => {$(
        impl $crate::math::Upcast for $t {
            type Target = $w;
            fn upcast(self) -> Self::Target {
                $w::from(self)
            }
        }
    )+};
}

upcast_impl!(i8 i16, i16 i32, i32 i64, i64 i128, u8 u16, u16 u32, u32 u64, u64 u128);

/// Numeric type.
pub trait Number:
    Sized
    + Copy
    + FromStr
    + Default
    + Debug
    + Display
    + Zero
    + One
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + RemAssign
    + PartialOrd
    + PartialEq
{
    fn new(n: usize) -> Self {
        (0..n).fold(Self::zero(), |acc, _| acc + Self::one())
    }
}

impl<T> Number for T where
    T: Sized
        + Copy
        + FromStr
        + Default
        + Debug
        + Display
        + Zero
        + One
        + Add<Output = Self>
        + AddAssign
        + Sub<Output = Self>
        + SubAssign
        + Mul<Output = Self>
        + MulAssign
        + Div<Output = Self>
        + DivAssign
        + Rem<Output = Self>
        + RemAssign
        + PartialOrd
        + PartialEq
{
}

/// Trait for types that can be inverted, returning an `Output` type.
pub trait Invertible {
    type Output;

    fn inverse(&self) -> Option<Self::Output>;
}

/// Trait for types that can be converted to a primitive underlying type.
pub trait AsPrimitive<T> {
    fn as_primitive(&self) -> T;
}

#[macro_export]
macro_rules! as_primitive_impl {
    ($($t: ident)+) => {$(
        impl $crate::math::AsPrimitive<$t> for $t {
            fn as_primitive(&self) -> $t {
                *self
            }
        }
    )+};
}

as_primitive_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

#[macro_export]
macro_rules! as_primitive_unsigned_impl {
    ($($t: ident)+) => {$(
        impl $crate::math::AsPrimitive<usize> for $t {
            fn as_primitive(&self) -> usize {
                *self as usize
            }
        }
    )+};
}

as_primitive_unsigned_impl!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128);
