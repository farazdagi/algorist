//! Root computation traits and implementations.
//!
//! Rust provides way to compute roots of floating-point numbers, but when it
//! comes to competitive programming, we often need to compute integer roots.
//! This module provides necessary traits and implementations for integer root
//! calculations.
//!
//! See, the [`IntRoot`] trait, which defines methods for computing integer
//! roots on most integer types in Rust.

use crate::math::{AsType, Integer};

/// Integer roots.
///
/// This trait provides methods to compute integer roots of a number.
pub trait IntRoot
where
    Self: Integer + AsType<f64>,
    f64: AsType<Self>,
{
    /// Checks if the number is a perfect power of `k` (`k >= 1`).
    ///
    /// A perfect power is a positive integer that can be expressed as an
    /// integer power of another positive integer greater than one.
    ///
    /// So, a number `n` is a perfect power if there exist integers `m > 1` and
    /// `k > 1` such that `n = m^k`.
    ///
    /// By convention, `0` is considered a perfect power for any `k >= 1`.
    ///
    /// # Examples
    ///
    /// ```
    /// use algorist::math::root::IntRoot;
    ///
    /// assert!(27.is_perfect_pow(3));
    /// assert!(!28.is_perfect_pow(3));
    /// assert!(81.is_perfect_pow(4));
    /// assert!(1.is_perfect_pow(128));
    ///
    /// assert!(0.is_perfect_pow(1));
    /// assert!(0.is_perfect_pow(2));
    /// assert!(0.is_perfect_pow(42));
    ///
    /// assert!((-8).is_perfect_pow(3));
    /// ```
    fn is_perfect_pow(&self, k: usize) -> bool {
        assert!(k >= 1);
        self.root(k).is_some()
    }

    /// Computes the `k`-th integer root of the number, if it exists.
    ///
    /// If a number is NOT a perfect power of `k`, this method returns `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use algorist::math::root::IntRoot;
    ///
    /// assert!(27.is_perfect_pow(3));
    /// assert_eq!(27.root(3), Some(3));
    /// assert!(!28.is_perfect_pow(3));
    /// assert_eq!(28.root(3), None);
    /// ```
    fn root(&self, k: usize) -> Option<Self> {
        let x = self.root_floor(k);
        if x.pow(k as u32) == *self {
            Some(x)
        } else {
            None
        }
    }

    /// Computes the square root of the number, if it exists.
    ///
    /// # Panics
    ///
    /// If the number is negative, this method panics.
    ///
    /// # Examples
    ///
    /// ```
    /// use algorist::math::root::IntRoot;
    ///
    /// assert_eq!(4.sqrt(), Some(2));
    /// assert_eq!(5.sqrt(), None);
    /// ```
    ///
    /// ``` should_panic
    /// use algorist::math::root::IntRoot;
    ///
    /// println!("Only imaginary roots for: {}", (-4).sqrt().unwrap());
    /// ```
    fn sqrt(&self) -> Option<Self> {
        assert!(
            self >= &Self::zero(),
            "Cannot compute square root of a negative number: {self}",
        );
        self.root(2)
    }

    /// Computes the cube root of the number, if it exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use algorist::math::root::IntRoot;
    ///
    /// assert_eq!(8.cbrt(), Some(2));
    /// assert_eq!((-8).cbrt(), Some(-2));
    /// ```
    fn cbrt(&self) -> Option<Self> {
        self.root(3)
    }

    /// Computes the `k`-th root of the number, rounded down.
    ///
    /// If you need to ensure that the root is an integer, use
    /// [`root`](IntRoot::root) method.
    ///
    /// # Panics
    ///
    /// Panics if `k < 1` or if `k` is even and the number is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use algorist::math::root::IntRoot;
    ///
    /// assert_eq!(27.root_floor(3), 3);
    /// assert_eq!(28.root_floor(3), 3);
    ///
    /// assert_eq!(-8.root_floor(3), -2);
    /// assert_eq!(-10.root_floor(3), -2);
    ///
    /// // r^2 <= x < (r+1)^2, since `x` is positive
    /// for x in 0..10 {
    ///     let r = x.root_floor(2);
    ///     assert!(r * r <= x && x < (r + 1) * (r + 1));
    /// }
    ///
    /// // (r - 1)^3 < x <= r^3, since `x` is negative
    /// // basically, rounding toward zero
    /// for x in -10..0 {
    ///     let r = x.root_floor(3);
    ///     assert!((r - 1) * (r - 1) * (r - 1) < x && x <= r * r * r);
    /// }
    /// ```
    #[must_use]
    fn root_floor(&self, k: usize) -> Self;

    /// Computes the `k`-th root of the number, rounded up.
    ///
    /// # Panics
    ///
    /// Panics if `k < 1` or if `k` is even and the number is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use algorist::math::root::IntRoot;
    ///
    /// assert_eq!(27.root_ceil(3), 3);
    /// assert_eq!(28.root_ceil(3), 4);
    /// ```
    #[must_use]
    fn root_ceil(&self, k: usize) -> Self {
        let n = *self;
        assert!(n >= Self::zero());
        let x = self.root_floor(k);
        if x.pow(k as u32) == n {
            x
        } else {
            x + Self::one()
        }
    }
}

#[macro_export]
macro_rules! impl_int_root_unsigned {
    ($($t: ident)+) => {$(
        impl $crate::math::root::IntRoot for $t {
            fn root_floor(&self, k: usize) -> Self {
                let n = *self;
                let mut x = ((n as f64).powf(1.0 / k as f64).floor()) as $t;
                while x.pow(k as u32) > n {
                    x -= 1;
                }
                x
            }
        }
    )+};
}

#[macro_export]
macro_rules! impl_int_root_signed {
    ($($t: ident $u: ident),+) => {$(
        impl $crate::math::root::IntRoot for $t {
            fn root_floor(&self, k: usize) -> Self {
                assert!(k >= 1);
                let n = *self;
                return if n >= 0 {
                     (n as $u).root_floor(k) as $t
                } else {
                    assert!(k.is_odd(), "Cannot compute even root of a negative number: {}", n);
                    -((n.wrapping_neg() as $u).root_floor(k) as $t)
                }
            }
        }
    )+};
}

impl_int_root_unsigned!(u16 u32 u64 usize);
impl_int_root_signed!(i16 u16, i32 u32, i64 u64, isize usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root() {
        assert_eq!((-8_i64).root(3), Some(-2));

        assert_eq!(27_i64.root(3), Some(3));
        assert_eq!(28_i64.root(3), None);
        assert_eq!(28_i64.root_floor(3), 3);
        assert_eq!(28_i64.root_ceil(3), 4);
        assert_eq!(29_i64.root_ceil(3), 4);

        assert_eq!(0_i64.root(2), Some(0));
        assert_eq!(1_i64.root(2), Some(1));
        assert_eq!(2_i64.root(2), None);
        assert_eq!(3_i64.root(2), None);
        assert_eq!(4_i64.root(2), Some(2));
        assert_eq!(12_i64.root(2), None);
        assert_eq!(12_i64.root_floor(2), 3);
        assert_eq!(12_i64.root_ceil(2), 4);

        let x: i32 = 12345;
        assert_eq!(x.root(1), Some(x));
        assert_eq!(x.root(2), x.sqrt());
        assert_eq!(x.root(3), x.cbrt());
        assert_eq!(x.root(4), None);
        assert_eq!(x.root_floor(4), 10);
        assert_eq!(x.root_floor(13), 2);
        assert_eq!(x.root_floor(14), 1);
        assert_eq!(x.root_floor(std::usize::MAX), 1);

        assert_eq!(std::i32::MAX.root_floor(30), 2);
        assert_eq!(std::i32::MAX.root_floor(31), 1);
        assert_eq!(std::i32::MIN.root_floor(31), -2);
        assert_eq!((std::i32::MIN + 1).root_floor(31), -1);

        assert_eq!(std::u32::MAX.root_floor(31), 2);
        assert_eq!(std::u32::MAX.root_floor(32), 1);
    }
}
