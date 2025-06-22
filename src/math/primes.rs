//! Functions for working with prime numbers.
//!
//! ## Primality test
//!
//! To check whether a number is prime, either use [`is_prime`], or the
//! [`IsPrime`] trait.
//!
//! ```
//! use algorist::math::primes::{IsPrime, is_prime};
//!
//! assert!(is_prime(29));
//! assert!(!is_prime(30));
//! assert!(29.is_prime());
//! assert!(!30.is_prime());
//!
//! // For larger numbers:
//! assert!(is_prime(1_000_000_007));
//! assert!(1_000_000_007.is_prime());
//! ```
//!
//! ## Sieve of Eratosthenes
//!
//! If you need to use a Sieve of Eratosthenes directly, rely on the [`sieve`]
//! function.
//!
//! To iterate over prime numbers up to a given limit (internally relies on the
//! Sieve of Eratosphenes), use the [`SieveIter`] and
//! [`Primes`] traits:
//!
//! ```
//! use algorist::math::primes::{Primes, SieveIter};
//!
//! // Create an iterator that yields primes up to and including `n`.
//! let primes: Vec<_> = SieveIter::new(30).collect::<Vec<_>>();
//! assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
//!
//! // You can also use the `Primes` trait to get an iterator over primes:
//! let primes: Vec<_> = 30.primes_iter().collect();
//! assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
//! let primes = 30.primes(); // more ergonomic way to get a vector of primes
//! assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
//! ```
//!
//! Alternatively, you can use the [`primes`] function to get a vector of
//! primes. For non-prime numbers, use [`non_primes`].
//!
//! ```
//! use algorist::math::primes::{non_primes, primes};
//!
//! assert_eq!(primes(23), vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
//! assert_eq!(primes(24), vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
//! assert_eq!(non_primes(18), vec![1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 18]);
//! assert_eq!(non_primes(19), vec![1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 18]);
//! ```
//!
//! ## Factorization
//!
//! ### Prime factorization
//!
//! To factorize a number into its prime factors, use the [`factorize`]
//! function.
//!
//! Alternatively, rely on [`PrimeFactorsIter`] iterator, either directly of via
//! [`PrimeFactors`] trait.
//!
//! The former returns an owned vector of [`PrimeFactor`] structs, and the
//! latter provides an *iterator* over them.
//!
//! ```
//! use algorist::math::primes::{PrimeFactor, PrimeFactors, PrimeFactorsIter, factorize};
//!
//! // Factorize a number into its prime factors.
//! let factors = factorize(30);
//! assert_eq!(factors, vec![
//!     PrimeFactor(2, 1),
//!     PrimeFactor(3, 1),
//!     PrimeFactor(5, 1),
//! ]);
//!
//! // Alternatively, use the iterator to get the same result:
//! let factors: Vec<PrimeFactor> = PrimeFactorsIter::new(30).collect();
//! assert_eq!(factors, vec![
//!     PrimeFactor(2, 1),
//!     PrimeFactor(3, 1),
//!     PrimeFactor(5, 1),
//! ]);
//!
//! // You can also use the `PrimeFactors` trait for convenience:
//! let factors = 30.prime_factors();
//! assert_eq!(factors, vec![
//!     PrimeFactor(2, 1),
//!     PrimeFactor(3, 1),
//!     PrimeFactor(5, 1),
//! ]);
//! let factors: Vec<_> = 30.prime_factors_iter().collect();
//! assert_eq!(factors, vec![
//!     PrimeFactor(2, 1),
//!     PrimeFactor(3, 1),
//!     PrimeFactor(5, 1),
//! ]);
//!
//! // For larger numbers (and different numeric types):
//! let factors = vec![PrimeFactor(1_000_000_007, 1)];
//! assert_eq!(factors, 1_000_000_007.prime_factors());
//! assert_eq!(factors, 1_000_000_007_i32.prime_factors());
//! assert_eq!(factors, 1_000_000_007_usize.prime_factors());
//! assert_eq!(factors, 1_000_000_007_u64.prime_factors());
//! ```
//!
//! ### Factors (not necessarily prime or proper)
//!
//! To get all factors of a number (not necessarily prime or proper), use the
//! [`factors`] function or the [`Factors`] trait.
//!
//! ```
//! use algorist::{
//!     ext::vec::sorted::Sorted,
//!     math::primes::{Factors, factors},
//! };
//!
//! assert_eq!(factors(30).sorted(), vec![1, 2, 3, 5, 6, 10, 15, 30]);
//! assert_eq!(30.factors().sorted(), vec![1, 2, 3, 5, 6, 10, 15, 30]);
//! ```

use crate::math::{AsPrimitive, Number};

/// Trait for types that can be checked for primality.
pub trait IsPrime {
    /// Returns whether the number is prime.
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::math::primes::IsPrime;
    ///
    /// let n = 29;
    /// assert!(n.is_prime());
    ///
    /// let m = 30;
    /// assert!(!m.is_prime());
    /// ```
    fn is_prime(self) -> bool;
}

impl<T: Number> IsPrime for T {
    fn is_prime(self) -> bool {
        is_prime(self)
    }
}

/// Returns whether the given number is prime.
///
/// # Example
///
/// ```
/// use algorist::math::primes::is_prime;
///
/// assert!(is_prime(2));
/// assert!(is_prime(3));
/// assert!(!is_prime(4));
/// assert!(is_prime(29));
///
/// // For larger numbers:
/// assert!(is_prime(1_000_000_007));
/// ```
pub fn is_prime<T: Number>(n: T) -> bool {
    if n <= T::one() {
        return false;
    }
    let mut i = T::new(2);
    while i * i <= n {
        if n % i == T::zero() {
            return false;
        }
        i += T::one();
    }
    true
}

/// Iterator over prime numbers up to a given limit using the Sieve of
/// Eratosthenes.
///
/// # Example
///
/// ```
/// use algorist::math::primes::SieveIter;
///
/// let primes: Vec<_> = SieveIter::new(30).collect();
/// assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
/// ```
pub struct SieveIter {
    n: usize,
    nsqrt: usize,
    current: usize,
    nums: Vec<bool>,
}

impl SieveIter {
    /// Creates a new iterator that yields primes up to and including `n`.
    pub fn new<T: Number + AsPrimitive<usize>>(n: T) -> Self {
        let n = n.as_primitive().max(2);
        let mut is_prime = vec![true; n + 1];
        is_prime[0] = false;
        is_prime[1] = false;
        Self {
            n,
            nsqrt: ((n as f64).sqrt() as usize),
            nums: is_prime,
            current: 2,
        }
    }
}

impl Iterator for SieveIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // If current exceeds the square root of n, we can skip to the end since all
        // remaining non-prime numbers are already marked.
        if self.current > self.nsqrt {
            return self
                .nums
                .iter()
                .skip(self.current)
                .position(|&x| x)
                .and_then(|i| {
                    // `i` is the index of the next prime number
                    let prime = self.current + i;
                    self.current = prime + 1; // Move current to the next number
                    Some(prime)
                });
        }

        // We haven't reached the square root of n yet, so we continue checking and
        // marking non-primes.
        while self.current <= self.nsqrt {
            let i = self.current;
            self.current += 1;
            if self.nums[i] {
                // Mark multiples of n as not prime
                for j in (i * i..=self.n).step_by(i) {
                    self.nums[j] = false;
                }
                return Some(i);
            }
        }
        None
    }
}

/// Computes the sieve of Eratosthenes up to (and including) `n`.
///
/// Returns a vector of booleans where the index represents the number and
/// the value indicates whether it is prime (`true`) or not (`false`).
///
/// # Example
///
/// ```
/// use algorist::math::primes::sieve;
///
/// let primes = sieve(30);
/// assert_eq!(primes[2], true);
/// assert_eq!(primes[3], true);
/// assert_eq!(primes[4], false);
/// // and so on...
/// ```
pub fn sieve<T: Number + AsPrimitive<usize>>(n: T) -> Vec<bool> {
    let n = n.as_primitive().max(2);
    let mut nums = vec![true; n + 1];
    nums[0] = false;
    nums[1] = false;
    for i in 2..=((n as f64).sqrt() as usize) {
        if nums[i] {
            for j in (i * i..=n).step_by(i) {
                nums[j] = false;
            }
        }
    }
    nums
}

/// Prime numbers up to `n`.
pub trait Primes: Sized {
    /// Returns an iterator over the prime numbers up to `n`.
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::math::primes::Primes;
    ///
    /// let primes: Vec<_> = 30.primes_iter().collect();
    /// assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    /// ```
    fn primes_iter(self) -> impl Iterator<Item = usize>;

    /// Returns a vector of prime numbers up to `n`.
    ///
    /// # Example
    /// ```
    /// use algorist::math::primes::Primes;
    ///
    /// let primes = 30.primes();
    /// assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    /// ```
    fn primes(self) -> Vec<usize> {
        self.primes_iter().collect()
    }
}

impl<T: Number + AsPrimitive<usize>> Primes for T {
    fn primes_iter(self) -> impl Iterator<Item = usize> {
        SieveIter::new(self)
    }
}

/// Computes and returns prime numbers up to `n`.
///
/// # Example
///
/// ```
/// use algorist::math::primes::primes;
///
/// let primes = primes(17);
/// assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17]);
/// ```
pub fn primes<T: Number + AsPrimitive<usize>>(n: T) -> Vec<usize> {
    sieve(n)
        .iter()
        .enumerate()
        .filter_map(|(i, &p)| if p { Some(i) } else { None })
        .collect()
}

/// Computes and returns non-prime numbers up to `n`.
///
/// # Example
///
/// ```
/// use algorist::math::primes::non_primes;
///
/// let non_primes = non_primes(18);
/// assert_eq!(non_primes, vec![1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 18]);
/// ```
pub fn non_primes<T: Number + AsPrimitive<usize>>(n: T) -> Vec<usize> {
    let primes = sieve(n);
    (1..=n.as_primitive()).filter(|&x| !primes[x]).collect()
}

/// Represents a prime factor and its count.
///
/// # Example
///
/// ```
/// use algorist::math::primes::PrimeFactor;
///
/// let factor = PrimeFactor::new(2, 3);
/// assert_eq!(factor.factor(), 2);
/// assert_eq!(factor.count(), 3);
///
/// let factor = PrimeFactor(3, 1);
/// assert_eq!(factor.factor(), 3);
/// assert_eq!(factor.count(), 1);
///
/// // Convert to tuple
/// let tuple: (usize, usize) = factor.into();
/// assert_eq!(tuple, (3, 1));
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PrimeFactor(pub usize, pub usize);

impl From<PrimeFactor> for (usize, usize) {
    fn from(factor: PrimeFactor) -> Self {
        (factor.0, factor.1)
    }
}

impl PrimeFactor {
    /// Creates a new `PrimeFactor` with the given factor and count.
    pub fn new(factor: usize, count: usize) -> Self {
        Self(factor, count)
    }

    /// Returns the factor.
    pub fn factor(&self) -> usize {
        self.0
    }

    /// Returns the count of the factor.
    pub fn count(&self) -> usize {
        self.1
    }
}

/// An iterator over the prime factors of a number.
///
/// # Example
/// ```
/// use algorist::math::primes::{PrimeFactor, PrimeFactorsIter};
///
/// let factors: Vec<PrimeFactor> = PrimeFactorsIter::new(30).collect();
/// assert_eq!(factors, vec![
///     PrimeFactor(2, 1),
///     PrimeFactor(3, 1),
///     PrimeFactor(5, 1),
/// ]);
/// ```
#[derive(Debug)]
pub struct PrimeFactorsIter {
    value: usize,
    factors: std::ops::RangeInclusive<usize>,
}

impl PrimeFactorsIter {
    pub fn new(n: usize) -> Self {
        Self {
            value: n,
            factors: 2..=((n as f64).sqrt() as usize),
        }
    }
}

impl Iterator for PrimeFactorsIter {
    type Item = PrimeFactor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value == 1 {
            return None;
        }

        for factor in self.factors.by_ref() {
            if self.value % factor == 0 {
                let mut count = 0;
                while self.value % factor == 0 {
                    self.value /= factor;
                    count += 1;
                }
                return Some(PrimeFactor(factor, count));
            }
        }

        if self.value > 1 {
            let factor = self.value;
            self.value = 1;
            Some(PrimeFactor(factor, 1))
        } else {
            None
        }
    }
}

/// Prime factors of a number.
///
/// # Example
/// ```
/// use algorist::math::primes::{PrimeFactor, PrimeFactors};
///
/// let factors: Vec<_> = 30.prime_factors_iter().collect();
/// assert_eq!(factors, vec![
///     PrimeFactor(2, 1),
///     PrimeFactor(3, 1),
///     PrimeFactor(5, 1),
/// ]);
///
/// let factors = 30.prime_factors();
/// assert_eq!(factors, vec![
///     PrimeFactor(2, 1),
///     PrimeFactor(3, 1),
///     PrimeFactor(5, 1),
/// ]);
///
/// // For larger numbers:
/// let factors = 1_000_000_000.prime_factors();
/// assert_eq!(factors, vec![PrimeFactor(2, 9), PrimeFactor(5, 9)]);
/// let factors = 1_000_000_007.prime_factors();
/// assert_eq!(factors, vec![PrimeFactor(1_000_000_007, 1)]);
/// ```
pub trait PrimeFactors: Number + AsPrimitive<usize> {
    /// Returns an iterator over the prime factors of the number.
    ///
    /// # Example
    /// ```
    /// use algorist::math::primes::{PrimeFactor, PrimeFactors};
    ///
    /// let factors: Vec<PrimeFactor> = 30.prime_factors_iter().collect();
    /// assert_eq!(factors, vec![
    ///     PrimeFactor(2, 1),
    ///     PrimeFactor(3, 1),
    ///     PrimeFactor(5, 1)
    /// ]);
    /// ```
    fn prime_factors_iter(self) -> PrimeFactorsIter;

    /// Returns the prime factorization of the number.
    ///
    /// # Example
    /// ```
    /// use algorist::math::primes::{PrimeFactor, PrimeFactors};
    ///
    /// let factors = 30.prime_factors();
    /// assert_eq!(factors, vec![
    ///     PrimeFactor(2, 1),
    ///     PrimeFactor(3, 1),
    ///     PrimeFactor(5, 1)
    /// ]);
    /// ```
    fn prime_factors(self) -> Vec<PrimeFactor> {
        self.prime_factors_iter().collect()
    }

    /// Computes the largest prime factor of each number up to `n`.
    ///
    /// Currently, it supports numbers up to 10,000.
    ///
    /// # Example
    /// ```
    /// use algorist::math::primes::PrimeFactors;
    ///
    /// let factors = 15.max_prime_factors();
    /// assert_eq!(factors, vec![
    ///     0, 0, 2, 3, 2, 5, 3, 7, 2, 3, 5, 11, 3, 13, 7, 5
    /// ]);
    /// assert_eq!(factors[2], 2); // meaning 2's largest prime factor is 2
    /// assert_eq!(factors[15], 5); // meaning 15's largest prime factor is 5
    /// ```
    fn max_prime_factors(self) -> Vec<usize> {
        let n = self.as_primitive();
        assert!(
            n <= 10_000,
            "n must be less than 10,000 to avoid excessive time complexity"
        );
        let mut nums = vec![0; n + 1];
        for i in 2..=n {
            if nums[i] == 0 {
                for j in (i..=n).step_by(i) {
                    nums[j] = i;
                }
            }
        }
        nums
    }

    /// Computes number of distinct prime divisors of each number up to `n`.
    ///
    /// # Example
    ///
    /// ```
    /// use algorist::math::primes::PrimeFactors;
    ///
    /// let factors = 15.count_prime_factors();
    /// assert_eq!(factors, vec![
    ///     0, 0, 1, 1, 1, 1, 2, 1, 1, 1, 2, 1, 2, 1, 2, 2
    /// ]);
    /// assert_eq!(factors[2], 1); // meaning 2 has 1 distinct prime factor
    /// assert_eq!(factors[15], 2); // meaning 15 has 2 distinct prime factors (3 and 5)
    /// ```
    fn count_prime_factors(self) -> Vec<usize> {
        let n = self.as_primitive();
        let mut nums = vec![0; n + 1];
        for i in 2..=n {
            if nums[i] == 0 {
                for j in (i..=n).step_by(i) {
                    nums[j] += 1;
                }
            }
        }
        nums
    }
}

impl<T: Number + AsPrimitive<usize>> PrimeFactors for T {
    fn prime_factors_iter(self) -> PrimeFactorsIter {
        PrimeFactorsIter::new(self.as_primitive())
    }
}

/// Returns the prime factorization of the given number.
///
/// # Example
/// ```
/// use algorist::math::primes::{PrimeFactor, factorize};
///
/// // 30 = 2^1 * 3^1 * 5^1
/// let factors = factorize(30);
/// assert_eq!(factors, vec![
///     PrimeFactor(2, 1),
///     PrimeFactor(3, 1),
///     PrimeFactor(5, 1),
/// ]);
///
/// // 60 = 2^2 * 3^1 * 5^1
/// let factors = factorize(60);
/// assert_eq!(factors, vec![
///     PrimeFactor(2, 2),
///     PrimeFactor(3, 1),
///     PrimeFactor(5, 1),
/// ]);
///
/// // 90 = 2^1 * 3^2 * 5^1
/// let factors = factorize(90);
/// assert_eq!(factors, vec![
///     PrimeFactor(2, 1),
///     PrimeFactor(3, 2),
///     PrimeFactor(5, 1),
/// ]);
///
/// /// For larger numbers:
/// let factors = factorize(1_000_000_000);
/// assert_eq!(factors, vec![PrimeFactor(2, 9), PrimeFactor(5, 9)]);
/// let factors = factorize(1_000_000_007);
/// assert_eq!(factors, vec![PrimeFactor(1_000_000_007, 1)]);
/// ```
pub fn factorize(n: usize) -> Vec<PrimeFactor> {
    PrimeFactorsIter::new(n).collect()
}

/// Factors (not necessarily prime or proper) of a number.
///
/// # Example
///
/// ```
/// use algorist::{ext::vec::sorted::Sorted, math::primes::Factors};
///
/// assert_eq!(30.factors().sorted(), vec![1, 2, 3, 5, 6, 10, 15, 30]);
/// assert_eq!(60.factors().sorted(), vec![
///     1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60
/// ]);
///
/// // For larger numbers:
/// let divs = 1_000_000_000.factors();
/// assert_eq!(divs.len(), 100);
/// ```
pub trait Factors: Number + AsPrimitive<usize> {
    /// Factors (not necessarily prime or proper) of a number.
    fn factors(self) -> Vec<usize> {
        factors(self.as_primitive())
    }
}

impl<T: Number + AsPrimitive<usize>> Factors for T {}

/// Factors (not necessarily prime or proper) of a number.
///
/// # Example
/// ```
/// use algorist::{ext::vec::sorted::Sorted, math::primes::factors};
///
/// assert_eq!(factors(1), vec![1]);
/// assert_eq!(
///     factors(30).sorted(),
///     vec![1, 2, 3, 5, 6, 10, 15, 30].sorted()
/// );
/// assert_eq!(
///     factors(60).sorted(),
///     vec![1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60].sorted()
/// );
///
/// // For larger numbers:
/// let divs = factors(1_000_000_000);
/// assert_eq!(divs.len(), 100);
/// ```
pub fn factors(n: usize) -> Vec<usize> {
    generate_divisors(factorize(n))
}

/// Generates all divisors from the prime factors.
fn generate_divisors(factors: Vec<PrimeFactor>) -> Vec<usize> {
    let factor_powers: Vec<Vec<_>> = factors
        .into_iter()
        .map(Into::into)
        .map(|(factor, cnt)| (0..=cnt).map(|i| factor.pow(i as u32)).collect::<Vec<_>>())
        .collect();

    generate_combinations(&factor_powers, 0, 1)
}

fn generate_combinations(factor_powers: &Vec<Vec<usize>>, i: usize, product: usize) -> Vec<usize> {
    if i == factor_powers.len() {
        vec![product]
    } else {
        let mut result = vec![];
        for &power in &factor_powers[i] {
            result.extend(generate_combinations(factor_powers, i + 1, product * power));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::ext::vec::sorted::Sorted};

    #[test]
    fn test_is_prime() {
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        let non_primes = [
            0, 1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25, 26, 27, 28,
        ];
        assert!(primes.iter().all(|&x| is_prime(x)));
        assert!(primes.iter().all(|&x| x.is_prime()));
        assert!(non_primes.iter().all(|&x| !is_prime(x)));
        assert!(non_primes.iter().all(|&x| !x.is_prime()));
    }

    #[test]
    fn test_sieve() {
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        let n = 30;
        let sieve = sieve(n);
        for i in 0..n {
            assert_eq!(sieve[i], primes.contains(&i));
        }
    }

    #[test]
    fn sieve_iter() {
        let iter = SieveIter::new(30);
        let primes: Vec<_> = iter.collect();
        assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn test_max_factors() {
        assert_eq!(30.max_prime_factors(), [
            0, 0, 2, 3, 2, 5, 3, 7, 2, 3, 5, 11, 3, 13, 7, 5, 2, 17, 3, 19, 5, 7, 11, 23, 3, 5, 13,
            3, 7, 29, 5
        ]);
    }

    #[test]
    fn test_count_factors() {
        assert_eq!(30.count_prime_factors(), [
            0, 0, 1, 1, 1, 1, 2, 1, 1, 1, 2, 1, 2, 1, 2, 2, 1, 1, 2, 1, 2, 2, 2, 1, 2, 1, 2, 1, 2,
            1, 3
        ]);
    }

    #[test]
    fn test_factorize() {
        assert_eq!(factorize(30), vec![
            PrimeFactor(2, 1),
            PrimeFactor(3, 1),
            PrimeFactor(5, 1),
        ]);
        assert_eq!(factorize(60), vec![
            PrimeFactor(2, 2),
            PrimeFactor(3, 1),
            PrimeFactor(5, 1),
        ]);
        assert_eq!(factorize(90), vec![
            PrimeFactor(2, 1),
            PrimeFactor(3, 2),
            PrimeFactor(5, 1),
        ]);
    }

    #[test]
    fn test_prime_factors_iter() {
        let mut factors = PrimeFactorsIter::new(30);
        assert_eq!(factors.next(), Some(PrimeFactor(2, 1)));
        assert_eq!(factors.next(), Some(PrimeFactor(3, 1)));
        assert_eq!(factors.next(), Some(PrimeFactor(5, 1)));
        assert_eq!(factors.next(), None);

        let mut factors = PrimeFactorsIter::new(60);
        assert_eq!(factors.next(), Some(PrimeFactor(2, 2)));
        assert_eq!(factors.next(), Some(PrimeFactor(3, 1)));
        assert_eq!(factors.next(), Some(PrimeFactor(5, 1)));
        assert_eq!(factors.next(), None);

        let mut factors = PrimeFactorsIter::new(90);
        assert_eq!(factors.next(), Some(PrimeFactor(2, 1)));
        assert_eq!(factors.next(), Some(PrimeFactor(3, 2)));
        assert_eq!(factors.next(), Some(PrimeFactor(5, 1)));
        assert_eq!(factors.next(), None);
    }

    #[test]
    fn test_all_divisors() {
        assert_eq!(30.factors().sorted(), vec![1, 2, 3, 5, 6, 10, 15, 30]);
        assert_eq!(60.factors().sorted(), vec![
            1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60
        ]);
        assert_eq!(90.factors().sorted(), vec![
            1, 2, 3, 5, 6, 9, 10, 15, 18, 30, 45, 90
        ]);

        assert_eq!(1.factors(), vec![1]);
    }

    #[test]
    fn test_big_prime() {
        assert!(is_prime(1_000_000_007));

        let factors = factorize(1_000_000_000);
        assert_eq!(factors, vec![PrimeFactor(2, 9), PrimeFactor(5, 9),]);

        let divs = 1_000_000_000.factors();
        assert_eq!(divs.len(), 100);
    }
}
