pub trait IntRoot: Sized {
    fn is_perfect_pow(self, k: usize) -> bool {
        self.root(k).is_some()
    }

    fn root(self, k: usize) -> Option<Self>;

    fn root_floor(self, k: usize) -> Self;

    fn root_ceil(self, k: usize) -> Self;
}

impl IntRoot for i64 {
    fn root(self, k: usize) -> Option<Self> {
        let x = self.root_floor(k);
        if x.pow(k as u32) == self {
            Some(x)
        } else {
            None
        }
    }

    fn root_floor(self, k: usize) -> Self {
        assert!(self >= 0);
        let mut x = (self as f64).powf(1.0 / k as f64).round() as i64;
        while x.pow(k as u32) > self {
            x -= 1;
        }
        x
    }

    fn root_ceil(self, k: usize) -> Self {
        assert!(self >= 0);
        let x = self.root_floor(k);
        if x.pow(k as u32) == self {
            x
        } else {
            x + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root() {
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
    }
}
