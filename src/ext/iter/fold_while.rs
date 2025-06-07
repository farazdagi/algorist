/// A fold operation that can be short-circuited.
pub enum FoldWhile<T> {
    Continue(T),
    Break(T),
}

impl<T> FoldWhile<T> {
    pub fn into_inner(self) -> T {
        match self {
            FoldWhile::Continue(t) | FoldWhile::Break(t) => t,
        }
    }
}

pub trait FoldWhileExt {
    type Item;

    fn fold_while<B, F>(&mut self, init: B, f: F) -> FoldWhile<B>
    where
        F: FnMut(B, Self::Item) -> FoldWhile<B>;
}

impl<I: Iterator> FoldWhileExt for I {
    type Item = I::Item;

    fn fold_while<B, F>(&mut self, mut init: B, mut f: F) -> FoldWhile<B>
    where
        F: FnMut(B, Self::Item) -> FoldWhile<B>,
    {
        while let Some(x) = self.next() {
            match f(init, x) {
                FoldWhile::Continue(new_init) => init = new_init,
                FoldWhile::Break(new_init) => return FoldWhile::Break(new_init),
            }
        }
        FoldWhile::Continue(init)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fold_while() {
        let v = vec![1, 2, 3, 4, 5];
        let res = v.into_iter().fold_while(0, |acc, x| {
            if x < 5 {
                FoldWhile::Continue(acc + x)
            } else {
                FoldWhile::Break(acc)
            }
        });
        assert_eq!(res.into_inner(), 10);
    }
}
