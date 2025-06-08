use std::collections::HashMap;

pub trait CountOccurrences<T> {
    /// Returns the number of occurrences of each element in the array.
    ///
    /// Basically an implementation of counting sort, where index position of
    /// resultant vector gives a value and the value at that index gives the
    /// count of that value in the array.
    ///
    /// Returned vector is of size `n + 1`, where `n` is the maximum value for
    /// which occurrence count is needed.
    fn count(&self, n: usize) -> Vec<usize>;

    /// Group by occurrence count.
    ///
    /// Additionally, if `exclude_zero` is set to true, the result will not
    /// consider occurrences of 0 (zero as element). Useful, if only natural
    /// numbers are supposed to be counted.
    fn group(&self, exclude_zero: bool) -> HashMap<usize, Vec<usize>>;
}

impl CountOccurrences<usize> for [usize] {
    fn count(&self, n: usize) -> Vec<usize> {
        let mut cnt = vec![0; n + 1];
        for i in 0..self.len() {
            if self[i] <= n {
                cnt[self[i]] += 1;
            }
        }
        cnt
    }

    fn group(&self, exclude_zero: bool) -> HashMap<usize, Vec<usize>> {
        self.iter()
            .enumerate()
            .skip(usize::from(exclude_zero))
            .fold(HashMap::new(), |mut acc, (val, &count)| {
                acc.entry(count).or_default().push(val);
                acc
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count() {
        let v = vec![1, 2, 3, 2, 0, 1, 3, 3, 2, 1];
        assert_eq!(v.count(3), vec![1, 3, 3, 3]);

        let v = vec![3, 2, 8, 3];
        assert_eq!(v.count(8), vec![0, 0, 1, 2, 0, 0, 0, 0, 1]);
        assert_eq!(v[..2].count(3), vec![0, 0, 1, 1]);
        assert_eq!(v[..3].count(8), vec![0, 0, 1, 1, 0, 0, 0, 0, 1]);
    }

    #[test]
    fn test_group() {
        let v = vec![1, 2, 3, 5, 2, 0, 1, 3, 3, 2, 1];
        let mut map = HashMap::new();
        map.insert(0, vec![4]);
        map.insert(1, vec![0, 5]);
        map.insert(3, vec![1, 2, 3]);
        assert_eq!(v.count(5).group(false), map);

        let mut map = HashMap::new();
        map.insert(0, vec![4]);
        map.insert(1, vec![5]);
        map.insert(3, vec![1, 2, 3]);
        assert_eq!(v.count(5).group(true), map);
    }
}
