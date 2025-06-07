use crate::math::{AsPrimitive, Number};

/// Returns whether the given number is prime.
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

/// Computes prime numbers up to `n` using the sieve of Eratosthenes.
pub fn sieve<T: Number + AsPrimitive<usize>>(n: T) -> Vec<bool> {
    let n = n.as_primitive().max(2);
    let mut nums = vec![true; n + 1];
    nums[0] = false;
    nums[1] = false;
    for i in 2..1 + (n as f64).sqrt() as usize {
        if nums[i] {
            for j in (i * i..=n).step_by(i) {
                nums[j] = false;
            }
        }
    }
    nums
}

/// Computes and returns prime numbers up to `n`.
pub fn sieved_primes<T: Number + AsPrimitive<usize>>(n: T) -> Vec<usize> {
    sieve(n)
        .iter()
        .enumerate()
        .filter_map(|(i, &p)| if p { Some(i) } else { None })
        .collect()
}

/// Computes and returns non-prime numbers up to `n`.
pub fn sieved_non_primes<T: Number + AsPrimitive<usize>>(n: T) -> Vec<usize> {
    let primes = sieve(n);
    (2..=n.as_primitive()).filter(|&x| !primes[x]).collect()
}

/// Computes the largest prime factor of each number up to `n`.
pub fn max_factors<T: Number + AsPrimitive<usize>>(n: T) -> Vec<usize> {
    let n = n.as_primitive();
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
pub fn count_factors<T: Number + AsPrimitive<usize>>(n: T) -> Vec<usize> {
    let n = n.as_primitive();
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

/// Represents a prime factor and its count.
#[derive(Debug, PartialEq)]
pub struct PrimeFactor(usize, usize);

impl Into<(usize, usize)> for PrimeFactor {
    fn into(self) -> (usize, usize) {
        (self.0, self.1)
    }
}

/// An iterator over the prime factors of a number.
#[derive(Debug)]
pub struct PrimeFactors {
    value: usize,
    factors: std::ops::Range<usize>,
}

impl PrimeFactors {
    pub fn new(n: usize) -> Self {
        Self {
            value: n,
            factors: 2..1 + (n as f64).sqrt() as usize,
        }
    }
}

impl Iterator for PrimeFactors {
    type Item = PrimeFactor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value == 1 {
            return None;
        }

        while let Some(factor) = self.factors.next() {
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

/// Returns the prime factorization of the given number.
pub fn factorize(n: usize) -> Vec<PrimeFactor> {
    PrimeFactors::new(n).collect()
}

/// Returns an iterator over the prime factorization of the given number.
pub fn factorize_iter(n: usize) -> PrimeFactors {
    PrimeFactors::new(n)
}

/// Returns all (not necessarily prime) divisors of the given number.
pub fn all_divisors(n: usize) -> Vec<usize> {
    generate_divisors(factorize(n))
}

/// Returns all (not necessarily prime) divisors of the given number in sorted
/// order.
pub fn all_divisors_sorted(n: usize) -> Vec<usize> {
    let mut divisors = generate_divisors(factorize(n));
    divisors.sort();
    divisors
}

/// Generates all divisors from the prime factors.
pub fn generate_divisors(factors: Vec<PrimeFactor>) -> Vec<usize> {
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
    use super::*;
    use crate::ext::vec::sorted::Sorted;

    #[test]
    fn test_is_prime() {
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
        let non_primes = [
            0, 1, 4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20, 21, 22, 24, 25, 26, 27, 28,
        ];
        assert!(primes.iter().all(|&x| is_prime(x)));
        assert!(non_primes.iter().all(|&x| !is_prime(x)));
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
    fn test_max_factors() {
        assert_eq!(max_factors(30), [
            0, 0, 2, 3, 2, 5, 3, 7, 2, 3, 5, 11, 3, 13, 7, 5, 2, 17, 3, 19, 5, 7, 11, 23, 3, 5, 13,
            3, 7, 29, 5
        ]);
    }

    #[test]
    fn test_count_factors() {
        assert_eq!(count_factors(30), [
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
    fn test_factorize_iter() {
        let mut factors = factorize_iter(30);
        assert_eq!(factors.next(), Some(PrimeFactor(2, 1)));
        assert_eq!(factors.next(), Some(PrimeFactor(3, 1)));
        assert_eq!(factors.next(), Some(PrimeFactor(5, 1)));
        assert_eq!(factors.next(), None);

        let mut factors = factorize_iter(60);
        assert_eq!(factors.next(), Some(PrimeFactor(2, 2)));
        assert_eq!(factors.next(), Some(PrimeFactor(3, 1)));
        assert_eq!(factors.next(), Some(PrimeFactor(5, 1)));
        assert_eq!(factors.next(), None);

        let mut factors = factorize_iter(90);
        assert_eq!(factors.next(), Some(PrimeFactor(2, 1)));
        assert_eq!(factors.next(), Some(PrimeFactor(3, 2)));
        assert_eq!(factors.next(), Some(PrimeFactor(5, 1)));
        assert_eq!(factors.next(), None);
    }

    #[test]
    fn test_all_divisors() {
        assert_eq!(all_divisors(30).sorted(), vec![1, 2, 3, 5, 6, 10, 15, 30]);
        assert_eq!(all_divisors(60).sorted(), vec![
            1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60
        ]);
        assert_eq!(all_divisors(90).sorted(), vec![
            1, 2, 3, 5, 6, 9, 10, 15, 18, 30, 45, 90
        ]);

        assert_eq!(all_divisors(1), vec![1]);
    }

    #[test]
    fn test_big_prime() {
        assert!(is_prime(1_000_000_007));

        let factors = factorize(1_000_000_000);
        assert_eq!(factors, vec![PrimeFactor(2, 9), PrimeFactor(5, 9),]);

        let divs = all_divisors_sorted(1_000_000_000);
        assert_eq!(divs.len(), 100);
    }
}
