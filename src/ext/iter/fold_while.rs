use std::ops::ControlFlow;

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
/// use {
///     algorist::ext::iter::fold_while::FoldWhileExt,
///     std::ops::ControlFlow::{Break, Continue},
/// };
///
/// let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
/// let res = v.into_iter().fold_while(0, |acc, x| {
///     if *x < 5 {
///         Continue(acc + x)
///     } else {
///         Break(acc)
///     }
/// });
///
/// assert_eq!(res, 10);
/// ```
pub trait FoldWhileExt {
    type Item;

    fn fold_while<B, F>(&mut self, init: B, f: F) -> B
    where
        F: FnMut(B, &Self::Item) -> ControlFlow<B, B>;
}

impl<I: Iterator> FoldWhileExt for I {
    type Item = I::Item;

    fn fold_while<B, F>(&mut self, mut init: B, mut f: F) -> B
    where
        F: FnMut(B, &Self::Item) -> ControlFlow<B, B>,
    {
        for x in self.by_ref() {
            match f(init, &x) {
                ControlFlow::Continue(new_init) => init = new_init,
                ControlFlow::Break(res) => return res,
            }
        }
        init
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_while() {
        use ControlFlow::{Break, Continue};
        let v = vec![1, 2, 3, 4, 5];
        let res = v.into_iter().fold_while(0, |acc, x| {
            if *x < 5 {
                Continue(acc + x)
            } else {
                Break(acc)
            }
        });
        assert_eq!(res, 10);
    }
}
