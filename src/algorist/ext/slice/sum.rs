use crate::math::Number;

pub trait MaxSum {
    type Output;

    fn max_sum(&self) -> Self::Output;
}

impl<T: Number + Ord> MaxSum for [T] {
    type Output = T;

    fn max_sum(&self) -> T {
        max_sum(self)
    }
}

/// Returns the maximum sum of a contiguous sub-array within the given array.
/// Implemented using Kadane's algorithm.
pub fn max_sum<T: Number + Ord>(arr: &[T]) -> T {
    let mut max_sum = T::zero();
    let mut current_sum = T::zero();

    for &num in arr {
        current_sum = current_sum.max(T::zero()) + num;
        max_sum = max_sum.max(current_sum);
    }
    max_sum
}

pub fn max_sum_from_iter<T: Number + Ord, I: Iterator<Item = T>>(iter: I) -> T {
    let mut max_sum = T::zero();
    let mut current_sum = T::zero();

    for num in iter {
        current_sum = current_sum.max(T::zero()) + num;
        max_sum = max_sum.max(current_sum);
    }
    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum() {
        assert_eq!(max_sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(max_sum(&[1, -2, 3, -4, 5]), 5);
        assert_eq!(max_sum(&[-1, -2, -3, -4, -5]), 0);
        assert_eq!(max_sum(&[1, 2, 3, 4, -1, 5, -1, -2, -3, -4, -5]), 14);
    }

    #[test]
    fn test_max_sum_trait() {
        assert_eq!([1, 2, 3, 4, 5].max_sum(), 15);
        assert_eq!([1, -2, 3, -4, 5].max_sum(), 5);
        assert_eq!([-1, -2, -3, -4, -5].max_sum(), 0);
        assert_eq!([1, 2, 3, 4, -1, 5, -1, -2, -3, -4, -5].max_sum(), 14);

        let arr = [1, 2, 3, 4, 5];
        assert_eq!(arr.max_sum(), 15);
    }

    #[test]
    fn test_max_sum_from_iter() {
        assert_eq!(max_sum_from_iter([1, 2, 3, 4, 5].iter().copied()), 15);
        assert_eq!(max_sum_from_iter([1, -2, 3, -4, 5].iter().copied()), 5);
        assert_eq!(max_sum_from_iter([-1, -2, -3, -4, -5].iter().copied()), 0);
        assert_eq!(
            max_sum_from_iter([1, 2, 3, 4, -1, 5, -1, -2, -3, -4, -5].iter().copied()),
            14
        );
    }
}
