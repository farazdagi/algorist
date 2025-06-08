pub trait IntLog: Sized {
    /// Returns the integer part of the logarithm of `self` in base `base`.
    #[must_use]
    fn log(self, base: Self) -> Self;

    /// How many times can the number be divided by `base` (integer division,
    /// fraction is dropped) before it becomes 0.
    #[must_use]
    fn div_till_zero(self, base: Self) -> Self;
}

impl IntLog for i64 {
    fn log(self, base: Self) -> Self {
        assert!(self > 0, "self must be a positive number");
        assert!(base > 1, "base must be greater than 1");
        (self as f64).log(base as f64).ceil() as Self
    }

    fn div_till_zero(self, base: Self) -> Self {
        assert!(self >= 0, "self must be non-negative number");
        assert!(base > 1, "base must be greater than 1");
        let mut x = self;
        let mut cnt = 0;
        while x > 0 {
            x /= base;
            cnt += 1;
        }
        cnt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log() {
        assert_eq!(1_i64.log(2), 0);
        assert_eq!(2_i64.log(2), 1);
        assert_eq!(3_i64.log(2), 2);
        assert_eq!(4_i64.log(2), 2);
        assert_eq!(5_i64.log(2), 3);
        assert_eq!(6_i64.log(2), 3);
        assert_eq!(7_i64.log(2), 3);
        assert_eq!(8_i64.log(2), 3);
        assert_eq!(9_i64.log(2), 4);
        assert_eq!(571_787_i64.log(83), 3);
        assert_eq!(50_000_000_i64.log(6), 10);
    }

    #[test]
    fn test_log_till_zero() {
        assert_eq!(0_i64.div_till_zero(2), 0);
        assert_eq!(1_i64.div_till_zero(2), 1);
        assert_eq!(2_i64.div_till_zero(2), 2);
        assert_eq!(3_i64.div_till_zero(2), 2);
        assert_eq!(4_i64.div_till_zero(2), 3);
        assert_eq!(5_i64.div_till_zero(2), 3);
        assert_eq!(6_i64.div_till_zero(2), 3);
        assert_eq!(7_i64.div_till_zero(2), 3);
        assert_eq!(8_i64.div_till_zero(2), 4);
        assert_eq!(9_i64.div_till_zero(2), 4);
        assert_eq!(571_787_i64.div_till_zero(83), 4);
        assert_eq!(50_000_000_i64.div_till_zero(6), 10);
    }
}
