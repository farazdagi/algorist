use std::ops::{Deref, DerefMut};

/// Result type for a fold operation that can be short-circuited.
///
/// This enum is used to represent the result of a fold operation that can be
/// continued or broken out of early.
///
/// It is particularly useful in with iterators where you want to accumulate a
/// value until a certain condition is met, at which point you can stop the
/// accumulation and return the accumulated value.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FoldWhile<T> {
    Continue(T),
    Break(T),
}

impl<T> Deref for FoldWhile<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Continue(t) | Self::Break(t) => t,
        }
    }
}

impl<T> DerefMut for FoldWhile<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Continue(t) | Self::Break(t) => t,
        }
    }
}

impl<T> FoldWhile<T> {
    pub fn into_inner(self) -> T {
        match self {
            Self::Continue(t) | Self::Break(t) => t,
        }
    }
}

/// Extension trait for iterators to provide a method for folding with early
/// termination.
///
/// This trait adds the `fold_while` method to any iterator, allowing it to
/// accumulate a value while a condition is met, and to stop the accumulation
/// when a certain condition is no longer satisfied.
///
/// # Example
///
/// ```
/// use algorist::ext::iter::fold_while::{FoldWhile, FoldWhileExt};
///
/// let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let res = v.into_iter().fold_while(0, |acc, x| {
///     if *x < 5 {
///         FoldWhile::Continue(acc + x)
///     } else {
///         FoldWhile::Break(acc)
///     }
/// });
///
/// assert_eq!(*res, 10);
/// assert_eq!(res.into_inner(), 10);
/// ```
pub trait FoldWhileExt {
    type Item;

    fn fold_while<B, F>(&mut self, init: B, f: F) -> FoldWhile<B>
    where
        F: FnMut(B, &Self::Item) -> FoldWhile<B>;
}

impl<I: Iterator> FoldWhileExt for I {
    type Item = I::Item;

    fn fold_while<B, F>(&mut self, mut init: B, mut f: F) -> FoldWhile<B>
    where
        F: FnMut(B, &Self::Item) -> FoldWhile<B>,
    {
        for x in self.by_ref() {
            match f(init, &x) {
                FoldWhile::Continue(new_init) => init = new_init,
                FoldWhile::Break(res) => return FoldWhile::Break(res),
            }
        }
        FoldWhile::Break(init)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_while() {
        use FoldWhile::{Break, Continue};
        let v = vec![1, 2, 3, 4, 5];
        let res = v.into_iter().fold_while(0, |acc, x| {
            if *x < 5 {
                Continue(acc + x)
            } else {
                Break(acc)
            }
        });
        assert_eq!(*res, 10);
    }
}
