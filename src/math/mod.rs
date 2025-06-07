pub mod gcd;
pub mod log;
pub mod modulo;
pub mod primes;
pub mod root;

use core::fmt::Display;
use std::convert::From;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};
use std::str::FromStr;

pub trait Value<T>: Copy + Clone + Eq + Ord + Default {
    fn val() -> T;
}

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
    ($name: ident: $t: ty = $val: expr) => {
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Default)]
        pub struct $name {}

        impl $crate::math::ConstValue<$t> for $name {
            const VAL: $t = $val;
        }
    };
}
pub use value_impl as value;

pub trait Zero {
    fn zero() -> Self;
}

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
                self as $w
            }
        }
    )+};
}

upcast_impl!(i8 i16, i16 i32, i32 i64, i64 i128, u8 u16, u16 u32, u32 u64, u64 u128);

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

pub trait Invertible {
    type Output;

    fn inverse(&self) -> Option<Self::Output>;
}

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
