//! Modular arithmetic
//!
//! Sometimes, especially in competitive programming, we need to perform
//! arithmetic operations under a modulo, for instance, the result is always
//! `actual_result % 1_000_000_007` i.e. result is always `< 1_000_000_007`.
//! This is useful for avoiding overflow when working with large numbers.
//!
//! # Mod7 (mod 1_000_000_007) arithmetic
//!
//! This module provides a [`Modulo`] type that represents numbers under a
//! certain modulo, with number of operations defined on it. By default, the
//! [`Mod7`] type is provided, which uses `1_000_000_007` as the modulo.
//!
//! ## Example
//!
//! ```
//! use algorist::math::modulo::{Mod7, Modulo};
//!
//! assert_eq!(Mod7::new(1_000_000_006).val(), 1_000_000_006);
//! assert_eq!(Mod7::new(1_000_000_007).val(), 0);
//! assert_eq!(Mod7::new(i64::MAX).val(), 291_172_003);
//!
//! assert_eq!(Mod7::new(1) + Mod7::new(2), Mod7::new(3));
//! assert_eq!(Mod7::new(1_000_000_006) + Mod7::new(1), Mod7::new(0));
//! ```
//!
//! To make it easier to work with, you can use the `ma!` (as in *m*odular
//! *a*rithmetic) macro to create a `Mod7` instance:
//!
//! ```
//! use algorist::math::modulo::{Mod7, ma};
//!
//! assert_eq!(ma!(42), Mod7::new(42));
//! assert_eq!(ma!(1_000_000_006).val(), 1_000_000_006);
//! assert_eq!(ma!(1_000_000_007).val(), 0);
//! assert_eq!(ma!(i64::MAX).val(), 291_172_003);
//!
//! assert_eq!(ma!(1) + ma!(2), ma!(3));
//! assert_eq!(ma!(1_000_000_006) + ma!(1), ma!(0));
//! ```
//!
//! # Custom modulo types
//!
//! You can define your own modulo types using the `modulo!` macro, which takes
//! the name of the type, the name of the constant value, the type of the
//! constant value, and the value of the constant.
//!
//! ## Example
//!
//! ```
//! use algorist::math::modulo::{modulo, modulo_alias, Modulo};
//!
//! modulo!(Mod13, Val13: i64 = 13);
//!
//! assert_eq!(Mod13::new(12).val(), 12);
//! assert_eq!(Mod13::new(13).val(), 0);
//! assert_eq!(Mod13::new(i64::MAX).val(), 7);
//!
//! assert_eq!(Mod13::new(1) + Mod13::new(2), Mod13::new(3));
//! assert_eq!(Mod13::new(12) + Mod13::new(1), Mod13::new(0));
//! assert_eq!(Mod13::new(12) - Mod13::new(1), Mod13::new(11));
//! assert_eq!(Mod13::new(12) * Mod13::new(2), Mod13::new(11));
//!
//! modulo_alias!(Mod13, ma);
//! assert_eq!(ma!(12) + ma!(2), ma!(1));
//! assert_eq!(ma!(12) * ma!(2), ma!(11));
//! ```

use {
    crate::{
        ext::slice::sum::{MaxSum, max_sum_from_iter},
        math::{ConstValue, Downcast, Invertible, Number, gcd::gcd_extended},
    },
    std::{
        cmp::PartialOrd,
        fmt::{Debug, Display},
        marker::PhantomData,
        ops::*,
        str::FromStr,
    },
};

/// A type representing numbers under a modulo `M`.
///
/// This type is generic over the number type `T` and a constant value type `M`
///
/// # Example
///
/// ```
/// use algorist::math::modulo::{Modulo};
/// use algorist::math::value;
///
/// value!(Val7: i64 = 1_000_000_007);
/// pub type Mod7 = Modulo<i64, Val7>;
///
/// assert_eq!(Mod7::new(1_000_000_006).val(), 1_000_000_006);
/// assert_eq!(Mod7::new(1_000_000_007).val(), 0);
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
pub struct Modulo<T, M: ConstValue<T>> {
    val: T,
    _phantom: PhantomData<M>,
}

impl<T: Number, M: ConstValue<T>> Modulo<T, M> {
    /// Creates a new `Modulo` instance without checking the value.
    ///
    /// # Panics
    ///
    /// If the value is not in the range `[0, M::val())`, it will panic.
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::math::modulo::Mod7;
    ///
    /// assert_eq!(Mod7::new_unchecked(1_000_000_006).val(), 1_000_000_006);
    /// ```
    ///
    /// The following will panic:
    ///
    /// ``` should_panic
    /// use algorist::math::modulo::Mod7;
    ///
    /// Mod7::new_unchecked(1_000_000_007);
    /// ```
    pub fn new_unchecked(val: T) -> Self {
        assert!(
            val >= T::zero() && val < M::val(),
            "Invalid modulo value: {val}"
        );
        Self {
            val,
            _phantom: PhantomData,
        }
    }

    /// Creates a new `Modulo` instance, checking the value.
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::math::modulo::Mod7;
    ///
    /// assert_eq!(Mod7::new(1_000_000_006).val(), 1_000_000_006);
    /// assert_eq!(Mod7::new(1_000_000_007).val(), 0);
    /// ```
    pub fn new(mut val: T) -> Self {
        if val < T::zero() {
            val += M::val();
            if val < T::zero() {
                val %= M::val();
                return Self::new(val);
            }
        } else if val >= M::val() {
            val -= M::val();
            if val >= M::val() {
                val %= M::val();
            }
        }
        Self::new_unchecked(val)
    }

    /// Returns the raw value of the modulo.
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::math::modulo::Mod7;
    ///
    /// assert_eq!(Mod7::new(1_000_000_006).val(), 1_000_000_006);
    /// assert_eq!(Mod7::new(1_000_000_007).val(), 0);
    /// ```
    pub fn val(&self) -> T {
        self.val
    }
}

impl<T, M> Modulo<T, M>
where
    T: Number + Downcast + BitAnd<Output = T> + ShrAssign<T>,
    T::Source: Number,
    M: ConstValue<T>,
{
    /// Raises the modulo number to the power of `exp`.
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::math::modulo::Mod7;
    ///
    /// assert_eq!(Mod7::new(2).pow(3).val(), 8);
    /// assert_eq!(Mod7::new(2).pow(1_000_000_006).val(), 1);
    /// ```
    #[must_use]
    pub fn pow(self, mut exp: T) -> Self {
        let mut result = Self::new(T::one());
        let mut base = self;
        while exp > T::zero() {
            if exp & T::one() == T::one() {
                result *= base;
            }
            base *= base;
            exp >>= T::one();
        }
        result
    }
}

impl<T: Number, M: ConstValue<T>> From<T> for Modulo<T, M> {
    fn from(num: T) -> Self {
        Self::new(num)
    }
}

impl<T: Number, M: ConstValue<T>> FromStr for Modulo<T, M> {
    type Err = <T as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        T::from_str(s).map(Self::new)
    }
}

impl<T: Number + Ord, M: ConstValue<T>> MaxSum for [Modulo<T, M>] {
    type Output = Modulo<T, M>;

    fn max_sum(&self) -> Self::Output {
        Modulo::<T, M>::from(max_sum_from_iter(self.iter().map(|m| m.val)))
    }
}

impl<T: Number, M: ConstValue<T>> Debug for Modulo<T, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.val, f)
    }
}

impl<T: Number, M: ConstValue<T>> Display for Modulo<T, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.val, f)
    }
}

impl<T, M> Invertible for Modulo<T, M>
where
    T: Number + Downcast,
    T::Source: Number,
    M: ConstValue<T>,
{
    type Output = Self;

    fn inverse(&self) -> Option<Self> {
        let (d, x, _) = gcd_extended(self.val, M::val());
        if d == T::one() {
            Some(Self::new(T::downcast(x % M::val().into())))
        } else {
            None
        }
    }
}

impl<T: Number, M: ConstValue<T>> Add for Modulo<T, M> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.val + rhs.val)
    }
}

impl<T: Number, M: ConstValue<T>> AddAssign for Modulo<T, M> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self::new(self.val + rhs.val);
    }
}

impl<T: Number, M: ConstValue<T>> Sub for Modulo<T, M> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.val - rhs.val)
    }
}

impl<T: Number, M: ConstValue<T>> SubAssign for Modulo<T, M> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self::new(self.val - rhs.val);
    }
}

impl<T, M> Mul for Modulo<T, M>
where
    T: Number + Downcast,
    T::Source: Number,
    M: ConstValue<T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(T::downcast(
            T::Source::from(self.val) * T::Source::from(rhs.val) % T::Source::from(M::val()),
        ))
    }
}

impl<T, M> MulAssign for Modulo<T, M>
where
    T: Number + Downcast,
    T::Source: Number,
    M: ConstValue<T>,
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self::new(T::downcast(
            T::Source::from(self.val) * T::Source::from(rhs.val) % T::Source::from(M::val()),
        ));
    }
}

impl<T, M> Div for Modulo<T, M>
where
    T: Number + Downcast,
    T::Source: Number,
    M: ConstValue<T>,
{
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self {
        self * rhs.inverse().expect("Division by zero")
    }
}

impl<T, M> DivAssign for Modulo<T, M>
where
    T: Number + Downcast,
    T::Source: Number,
    M: ConstValue<T>,
{
    #[allow(clippy::suspicious_op_assign_impl)]
    fn div_assign(&mut self, rhs: Self) {
        *self *= rhs.inverse().expect("Division by zero");
    }
}

impl<T: Number, M: ConstValue<T>> Neg for Modulo<T, M> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(M::val() - self.val)
    }
}

#[macro_export]
macro_rules! modulo_alias_impl {
    ($name:ident, $macro_name:ident) => {
        #[allow(non_local_definitions)]
        #[macro_export]
        macro_rules! $macro_name {
            ($val: expr) => {
                $name::new($val)
            };
        }
        pub use $macro_name;
    };
}
pub use modulo_alias_impl as modulo_alias;

#[macro_export]
macro_rules! modulo_impl {
    ($name:ident, $vname:ident : $t:ty = $val:expr) => {
        $crate::algorist::math::value!($vname: $t = $val);
        pub type $name = $crate::algorist::math::modulo::Modulo<$t, $vname>;
    };
}
pub use modulo_impl as modulo;

modulo!(Mod7, Val7: i64 = 1_000_000_007);
modulo_alias_impl!(Mod7, ma);

#[cfg(test)]
#[cfg(feature = "unit_tests")]
mod tests {
    use {super::*, std::i64};

    #[test]
    fn modulo_creation() {
        let test_cases = vec![
            (-1, 1_000_000_006),
            (-2_000_000_014, 0),
            (-2_000_000_013, 1),
            (i64::MIN, 708_828_003),
            (0, 0),
            (1_000_000_006, 1_000_000_006),
            (1_000_000_007, 0),
            (i64::MAX, i64::MAX % Val7::val()),
        ];

        for &(val, expected) in test_cases.iter() {
            let m = Mod7::new(val);
            assert_eq!(m.val, expected, "new()");
        }

        for (val, expected) in test_cases {
            let m = Mod7::from(val);
            assert_eq!(m.val, expected, "from()");
        }
    }

    #[test]
    fn modulo_addition() {
        let test_cases = vec![
            (1, 2, 3),
            (1_000_000_006, 1, 0),
            (1_000_000_006, 1_000_000_006, 1_000_000_005),
            (1_000_000_006, 1_000_000_007, 1_000_000_006),
            (1_000_000_007, 1_000_000_007, 0),
            (1_000_000_007, 1_000_000_008, 1),
            (i64::MAX, 1, i64::MAX % Val7::val() + 1),
            (i64::MAX, 1_000_000_007, i64::MAX % Val7::val()),
            (i64::MAX, 1_000_000_008, i64::MAX % Val7::val() + 1),
            (i64::MAX, i64::MAX, i64::MAX % Val7::val() * 2 % Val7::val()),
            (-1, 1, 0),
            (-1, -1, 1_000_000_005),
            (-1, -2, 1_000_000_004),
            (-1, -1_000_000_007, 1_000_000_006),
            (-1, -1_000_000_008, 1_000_000_005),
            (-1, i64::MIN, 708_828_002),
            (i64::MIN, i64::MIN, 417_655_999),
            (-1, -1_000_000_007, 1_000_000_006),
            (-1, -1_000_000_008, 1_000_000_005),
        ];

        for &(a, b, expected) in &test_cases {
            let m = Mod7::new(a) + Mod7::new(b);
            assert_eq!(m.val, expected, "add()");
        }

        for (a, b, expected) in test_cases {
            let mut m = Mod7::new(a);
            m += Mod7::new(b);
            assert_eq!(m.val, expected, "add_assign()");
        }
    }

    #[test]
    fn modulo_subtraction() {
        let test_cases = vec![
            (1, 2, 1_000_000_006),
            (1_000_000_006, 1, 1_000_000_005),
            (1_000_000_006, 1_000_000_006, 0),
            (1_000_000_006, 1_000_000_007, 1_000_000_006),
            (1_000_000_007, 1_000_000_007, 0),
            (1_000_000_007, 1_000_000_008, 1_000_000_006),
            (i64::MAX, 1, i64::MAX % Val7::val() - 1),
            (i64::MAX, 1_000_000_007, i64::MAX % Val7::val()),
            (i64::MAX, 1_000_000_008, i64::MAX % Val7::val() - 1),
            (i64::MAX, i64::MAX, 0),
            (-1, 1, 1_000_000_005),
            (-1, -1, 0),
            (-1, -2, 1),
            (-1, -1_000_000_007, 1_000_000_006),
            (-1, -1_000_000_008, 0),
            (-1, i64::MIN, 291_172_003),
            (i64::MIN, i64::MIN, 0),
            (-1, -1_000_000_007, 1_000_000_006),
            (-1, -1_000_000_008, 0),
        ];

        for &(a, b, expected) in &test_cases {
            let m = Mod7::new(a) - Mod7::new(b);
            assert_eq!(m.val, expected, "sub()");
        }

        for (a, b, expected) in test_cases {
            let mut m = Mod7::new(a);
            m -= Mod7::new(b);
            assert_eq!(m.val, expected, "sub_assign()");
        }
    }

    #[test]
    fn modulo_multiplication() {
        let test_cases = vec![
            (1, 2, 2),
            (1_000_000_006, 1, 1_000_000_006),
            (1_000_000_006, 2, 1_000_000_005),
            (1_000_000_006, 1_000_000_006, 1),
            (1_000_000_006, 1_000_000_007, 0),
            (1_000_000_007, 1_000_000_007, 0),
            (1_000_000_007, 1_000_000_008, 0),
            (i64::MAX, 1, i64::MAX % Val7::val()),
            (i64::MAX, 1_000_000_007, 0),
            (i64::MAX, 1_000_000_008, 291_172_003),
            (i64::MAX, i64::MAX, 737_564_071),
            (-1, 1, 1_000_000_006),
            (-1, -1, 1),
            (-1, -2, 2),
            (-1, -1_000_000_007, 0),
            (-1, -1_000_000_008, 1),
            (-1, i64::MIN, 291_172_004),
            (i64::MIN, i64::MIN, 319_908_071),
            (-1, -1_000_000_007, 0),
            (-1, -1_000_000_008, 1),
        ];

        for &(a, b, expected) in &test_cases {
            let m = Mod7::new(a) * Mod7::new(b);
            assert_eq!(m.val, expected, "mul()");
        }

        for (a, b, expected) in test_cases {
            let mut m = Mod7::new(a);
            m *= Mod7::new(b);
            assert_eq!(m.val, expected, "mul_assign()");
        }
    }

    #[test]
    fn modulo_inverse() {
        let test_cases = vec![
            (1, 1),
            (2, 500000004),
            (1_000_000_006, 1_000_000_006),
            (1_000_000_008, 1),
            (i64::MAX, 933_137_596),
            (-1, 1_000_000_006),
            (-2, 500000003),
            (-1_000_000_008, 1_000_000_006),
        ];

        for &(val, expected) in &test_cases {
            let m = Mod7::new(val);
            let inv = m.inverse().unwrap();
            assert_eq!(inv.val, expected, "inverse()");
            assert_eq!(m * inv, Mod7::new(1), "inverse()");
        }
    }

    #[test]
    fn modulo_division() {
        let test_cases = vec![
            (1, 1, 1),
            (2, 2, 1),
            (1_000_000_006, 1_000_000_006, 1),
            (1_000_000_008, 2, 500_000_004),
            (i64::MAX, 2, 645_586_005),
            (-1, 1, 1_000_000_006),
            (-2, 2, 1_000_000_006),
            (-1_000_000_008, 2, 500_000_003),
        ];

        for &(a, b, expected) in &test_cases {
            let m = Mod7::new(a) / Mod7::new(b);
            assert_eq!(m.val, expected, "div()");
        }

        for (a, b, expected) in test_cases {
            let mut m = Mod7::new(a);
            m /= Mod7::new(b);
            assert_eq!(m.val, expected, "div_assign()");
        }
    }

    #[test]
    fn modulo_negation() {
        let test_cases = vec![
            (1, 1_000_000_006),
            (1_000_000_006, 1),
            (1_000_000_008, 1_000_000_006),
            (i64::MAX, 708_828_004),
            (-1, 1),
            (-2, 2),
            (-1_000_000_008, 1),
        ];

        for &(val, expected) in &test_cases {
            let m = -Mod7::new(val);
            assert_eq!(m.val, expected, "neg()");
        }
    }

    #[test]
    fn modulo_pow() {
        let test_cases = vec![
            (1, 0i64, 1),
            (1, 1, 1),
            (1, 2, 1),
            (1, 1_000_000_006, 1),
            (1, 1_000_000_008, 1),
            (1, i32::MAX as i64, 1),
            (2, 1, 2),
            (2, 5, 32),
            (2, 1_000_000_006, 1),
            (2, 1_000_000_008, 4),
            (2, i32::MAX as i64, 914_893_544),
            (i64::MAX, 1, 291_172_003),
            (i64::MAX, 2, 737_564_071),
            (i64::MAX, 1_000_000_006, 1),
            (i64::MAX, 1_000_000_008, 737_564_071),
            (i64::MAX, i32::MAX as i64, 840_154_026),
            (-1, 1, 1_000_000_006),
            (-1, 2, 1),
            (-1, 1_000_000_006, 1),
            (-1, i32::MAX as i64, 1_000_000_006),
            (-2, 1, 1_000_000_005),
            (-2, 5, 999_999_975),
            (-2, 1_000_000_006, 1),
            (-2, 1_000_000_008, 4),
            (-i64::MAX, 10, 394_962_753),
            (-i64::MAX, 1_000_000_006, 1),
            (-i64::MAX, 1_000_000_008, 737_564_071),
            (-i64::MAX, i32::MAX as i64, 159_845_981),
        ];

        for &(base, exp, expected) in &test_cases {
            let m = Mod7::new(base).pow(exp);
            assert_eq!(m.val, expected, "pow()");
        }
    }

    #[test]
    fn modulo_from_str() {
        let test_cases = vec![
            ("0".to_string(), 0),
            ("1".to_string(), 1),
            ("1000000006".to_string(), 1_000_000_006),
            ("1000000007".to_string(), 0),
            ("1000000008".to_string(), 1),
            ("1000000009".to_string(), 2),
            ("1000000010".to_string(), 3),
            (format!("{}", i64::MAX), i64::MAX % Val7::val()),
        ];

        for (s, expected) in test_cases {
            let m: Mod7 = s.parse().unwrap();
            assert_eq!(m.val, expected, "from_str()");
        }
    }

    #[test]
    fn modulo_max_sum() {
        let test_cases = vec![
            (vec![1, 2, 3, 4, 5], 15),
            (vec![1, -2, 3, -4, 5], 3),
            (vec![-1, -2, -3, -4, -5], 999999992),
            (vec![1, 2, 3, 4, -1, 5, -1, -2, -3, -4, -5], 1000000006),
        ];

        for (arr, expected) in test_cases {
            let arr: Vec<Mod7> = arr.into_iter().map(Mod7::new).collect();
            let m = arr.max_sum();
            assert_eq!(m.val, expected, "max_sum()");
        }
    }

    #[test]
    fn custom_modulo() {
        modulo!(Mod13, Val13: i64 = 13);

        assert_eq!(Mod13::new(12).val(), 12);
        assert_eq!(Mod13::new(13).val(), 0);
        assert_eq!(Mod13::new(i64::MAX).val(), 7);

        assert_eq!(Mod13::new(1) + Mod13::new(2), Mod13::new(3));
        assert_eq!(Mod13::new(12) + Mod13::new(1), Mod13::new(0));
        assert_eq!(Mod13::new(12) - Mod13::new(1), Mod13::new(11));
        assert_eq!(Mod13::new(12) * Mod13::new(2), Mod13::new(11));

        modulo_alias!(Mod13, ma13);
        assert_eq!(ma13!(12) * ma13!(2), ma13!(11));
    }
}
