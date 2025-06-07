pub trait Rotated {
    fn rotated_left(self, k: usize) -> Self;
    fn rotated_right(self, k: usize) -> Self;
}

impl<T> Rotated for Vec<T> {
    fn rotated_left(mut self, k: usize) -> Self {
        self.rotate_left(k);
        self
    }

    fn rotated_right(mut self, k: usize) -> Self {
        self.rotate_right(k);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
   
    #[test]
    fn test_rotated_left() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(v.rotated_left(2), vec![3, 4, 5, 1, 2]);
    }
    
    #[test]
    fn test_rotated_right() {
        let v = vec![1, 2, 3, 4, 5];
        assert_eq!(v.rotated_right(2), vec![4, 5, 1, 2, 3]);
    }
}
