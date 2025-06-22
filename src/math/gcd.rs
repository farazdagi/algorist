//! Greatest Common Divisor (GCD) and Least Common Multiple (LCM) calculations.
//!
//! This module provides functions to compute the GCD and LCM of two numbers,
//!
//! # Example
//! ```
//! use algorist::math::gcd::{gcd, lcm};
//!
//! assert_eq!(gcd(240, 46), 2);
//! assert_eq!(gcd(240, 480), 240);
//! assert_eq!(gcd(0, 5), 5);
//! assert_eq!(gcd(7, 5), 1);
//!
//! assert_eq!(lcm(5, 7), 35);
//! assert_eq!(lcm(5, 0), 0);
//! assert_eq!(lcm(24, 36), 72);
//!
//! let (a, b) = (240, 46);
//! assert_eq!(gcd(a, b) * lcm(a, b), a * b);
//! ```

use {
    crate::math::{Downcast, Number, One, Zero},
    std::mem::swap,
};

/// Extended Euclidean algorithm.
///
/// The algorithm finds, in addition to `GCD`, the integers `x` and `y` such
/// that `a * x + b * y = gcd(a, b)`, i.e. the Bézout's identity's coefficients.
///
/// # Example
/// ```
/// use algorist::math::gcd::gcd_extended;
///
/// let (a, b): (i64, i64) = (240, 46);
/// assert_eq!(gcd_extended(a, b), (2, -9, 47));
/// ```
#[allow(clippy::many_single_char_names)]
pub fn gcd_extended<T>(a: T, b: T) -> (T, T::Source, T::Source)
where
    T: Number + Downcast,
    T::Source: Number,
{
    if b == T::zero() {
        return (a, T::Source::one(), T::Source::zero());
    }
    let (d, x, y) = gcd_extended(b, a % b);
    (d, y, x - T::Source::from(a / b) * y)
}

/// Computes the greatest common divisor (GCD) of two numbers.
///
/// # Example
///
/// ```
/// use algorist::math::gcd::gcd;
///
/// assert_eq!(gcd(240, 46), 2);
/// assert_eq!(gcd(0, 5), 5);
/// assert_eq!(gcd(42, 1_000_000_007), 1);
/// ```
pub fn gcd<T: Number>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        a %= b;
        swap(&mut a, &mut b);
    }
    a
}

/// Computes the least common multiple (LCM) of two numbers.
///
/// # Example
/// ```
/// use algorist::math::gcd::lcm;
///
/// assert_eq!(lcm(5, 7), 35);
/// assert_eq!(lcm(5, 0), 0);
/// assert_eq!(lcm(24, 36), 72);
/// ```
pub fn lcm<T: Number>(a: T, b: T) -> T {
    a / gcd(a, b) * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_extended() {
        let (d, x, y) = gcd_extended(240, 46);
        assert_eq!(d, 2);
        assert_eq!(x, -9i64);
        assert_eq!(y, 47i64);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(240, 46), 2);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(5, 7), 35);
        assert_eq!(lcm(5, 0), 0);
        assert_eq!(lcm(24, 36), 72);
    }
}
