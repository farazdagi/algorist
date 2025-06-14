use std::ops::ControlFlow;

/// Extension trait for objects (iterators, vectors, slices) to get a sliding
/// window of size 2.
pub trait SlidingWindowExt {
    type Item: Copy;
    type Iter: Iterator<Item = Self::Item>;

    fn sliding_window(self) -> SlidingWindow<Self::Iter>;

    fn sliding_window_try_fold<B, F>(self, init: B, mut f: F) -> B
    where
        Self: Sized,
        F: FnMut(B, Self::Item, Self::Item) -> ControlFlow<B, B>,
    {
        match self
            .sliding_window()
            .try_fold(init, move |acc, (a, b)| f(acc, a, b))
        {
            ControlFlow::Continue(acc) | ControlFlow::Break(acc) => acc,
        }
    }
}

impl<'a, T> SlidingWindowExt for std::slice::Iter<'a, T> {
    type Item = &'a T;
    type Iter = Self;

    fn sliding_window(self) -> SlidingWindow<Self::Iter> {
        SlidingWindow::new(self)
    }
}

impl<T: Copy> SlidingWindowExt for std::vec::IntoIter<T> {
    type Item = T;
    type Iter = Self;

    fn sliding_window(self) -> SlidingWindow<Self::Iter> {
        SlidingWindow::new(self)
    }
}

impl<'a, T: Copy + 'a> SlidingWindowExt for &'a Vec<T> {
    type Item = &'a T;
    type Iter = std::slice::Iter<'a, T>;

    fn sliding_window(self) -> SlidingWindow<Self::Iter> {
        SlidingWindow::new(self.iter())
    }
}

impl<'a, T> SlidingWindowExt for &'a [T] {
    type Item = &'a T;
    type Iter = std::slice::Iter<'a, T>;

    fn sliding_window(self) -> SlidingWindow<Self::Iter> {
        SlidingWindow::new(self.iter())
    }
}

impl SlidingWindowExt for std::str::Chars<'_> {
    type Item = char;
    type Iter = Self;

    fn sliding_window(self) -> SlidingWindow<Self::Iter> {
        SlidingWindow::new(self)
    }
}

impl<I: Iterator> SlidingWindowExt for SlidingWindow<I>
where
    I::Item: Copy,
{
    type Item = I::Item;
    type Iter = I;

    fn sliding_window(self) -> SlidingWindow<Self::Iter> {
        self
    }
}

/// Sliding window of size 2.
pub struct SlidingWindow<I: Iterator>
where
    I::Item: Copy,
{
    iter: I,
    prev: Option<I::Item>,
}

impl<I: Iterator> SlidingWindow<I>
where
    I::Item: Copy,
{
    pub fn new(iter: I) -> Self {
        Self { iter, prev: None }
    }
}

impl<I: Iterator> Iterator for SlidingWindow<I>
where
    I::Item: Copy,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iter.next()?;
        if self.prev.is_none() {
            self.prev = Some(next);
            return self.next();
        }

        let result = self.prev.take().map(|prev| (prev, next));
        self.prev = Some(next);
        result
    }
}

impl<I: Iterator> From<I> for SlidingWindow<I>
where
    I::Item: Copy,
{
    fn from(iter: I) -> Self {
        Self::new(iter)
    }
}

#[cfg(test)]
#[cfg(feature = "unit_tests")]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let v = vec![1, 2, 3, 4, 5];
        let mut w = SlidingWindow::from(v.iter());
        assert_eq!(w.next(), Some((&1, &2)));
        assert_eq!(w.next(), Some((&2, &3)));
        assert_eq!(w.next(), Some((&3, &4)));
        assert_eq!(w.next(), Some((&4, &5)));
        assert_eq!(w.next(), None);

        let mut w = SlidingWindow::from(v.into_iter());
        assert_eq!(w.next(), Some((1, 2)));
        assert_eq!(w.next(), Some((2, 3)));
        assert_eq!(w.next(), Some((3, 4)));
        assert_eq!(w.next(), Some((4, 5)));
        assert_eq!(w.next(), None);
    }

    #[test]
    fn test_iter_window() {
        let v = vec![1, 2, 3, 4, 5];
        let mut w = v.iter().sliding_window();
        assert_eq!(w.next(), Some((&1, &2)));
        assert_eq!(w.next(), Some((&2, &3)));
        assert_eq!(w.next(), Some((&3, &4)));
        assert_eq!(w.next(), Some((&4, &5)));
        assert_eq!(w.next(), None);

        let mut w = v.into_iter().sliding_window();
        assert_eq!(w.next(), Some((1, 2)));
        assert_eq!(w.next(), Some((2, 3)));
        assert_eq!(w.next(), Some((3, 4)));
        assert_eq!(w.next(), Some((4, 5)));
        assert_eq!(w.next(), None);

        let v = vec![1, 2, 3, 4, 5];
        let mut w = v
            .iter()
            .sliding_window()
            .map(|(a, b)| a + b)
            .filter(|&x| x != 5);
        assert_eq!(w.next(), Some(3));
        assert_eq!(w.next(), Some(7));
        assert_eq!(w.next(), Some(9));
        assert_eq!(w.next(), None);

        let v = vec![1, 2, 3, 4, 5];
        let mut w = v.sliding_window().map(|(a, b)| a + b).filter(|&x| x != 5);
        assert_eq!(w.next(), Some(3));
        assert_eq!(w.next(), Some(7));
        assert_eq!(w.next(), Some(9));
        assert_eq!(w.next(), None);
    }

    #[test]
    fn test_iter_window_try_fold() {
        let v: Vec<i32> = vec![];
        let reps = v.sliding_window_try_fold(1, |acc, a, b| {
            if a == b {
                ControlFlow::Continue(acc + 1)
            } else {
                ControlFlow::Break(acc)
            }
        });
        assert_eq!(reps, 1);

        let v: Vec<i32> = vec![12];
        let reps = v.sliding_window_try_fold(1, |acc, a, b| {
            if a == b {
                ControlFlow::Continue(acc + 1)
            } else {
                ControlFlow::Break(acc)
            }
        });
        assert_eq!(reps, 1);

        let v = vec![1, 1, 2, 3, 4, 5];
        let reps = v.sliding_window_try_fold(1, |acc, a, b| {
            if a == b {
                ControlFlow::Continue(acc + 1)
            } else {
                ControlFlow::Break(acc)
            }
        });
        assert_eq!(reps, 2);

        let s = "aaabcc";
        let reps = s.chars().sliding_window_try_fold(1, |acc, a, b| {
            if a == b {
                ControlFlow::Continue(acc + 1)
            } else {
                ControlFlow::Break(acc)
            }
        });
        assert_eq!(reps, 3);
    }
}
