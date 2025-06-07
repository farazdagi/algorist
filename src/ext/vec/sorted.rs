pub trait Sorted<T> {
    fn sorted(self) -> Self;

    fn sorted_by_key<K, F>(self, f: F) -> Self
    where
        K: Ord,
        F: FnMut(&T) -> K;

    fn sorted_dedup(self) -> Self;
}

impl<T: Ord> Sorted<T> for Vec<T> {
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }

    fn sorted_by_key<K, F>(mut self, f: F) -> Self
    where
        K: Ord,
        F: FnMut(&T) -> K,
    {
        self.sort_by_key(f);
        self
    }

    fn sorted_dedup(mut self) -> Self {
        self.sort();
        self.dedup();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted() {
        let v = vec![3, 2, 1];
        assert_eq!(v.sorted(), vec![1, 2, 3]);
    }

    #[test]
    fn test_sorted_by_key() {
        let v = vec!["bb", "a", "ccc"];
        assert_eq!(v.sorted_by_key(|s| s.len()), vec!["a", "bb", "ccc"]);
    }

    #[test]
    fn test_sorted_dedup() {
        let v = vec![1, 2, 2, 3, 1];
        assert_eq!(v.sorted_dedup(), vec![1, 2, 3]);
    }
}
