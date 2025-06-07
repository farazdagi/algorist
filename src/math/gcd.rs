use std::mem::swap;

use super::{Downcast, Number, One, Zero};

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

pub fn gcd<T: Number>(mut a: T, mut b: T) -> T {
    while b != T::zero() {
        a %= b;
        swap(&mut a, &mut b);
    }
    a
}

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
