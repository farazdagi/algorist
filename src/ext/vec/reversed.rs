pub trait Reversed {
    #[must_use]
    fn reversed(self) -> Self;
}

impl<T> Reversed for Vec<T> {
    fn reversed(mut self) -> Self {
        self.reverse();
        self
    }
}
