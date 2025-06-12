pub trait TupleTransform<F, R>
where
    Self: Sized,
    F: FnOnce(Self) -> R,
{
    fn transform(self, f: F) -> R;
}

impl<F, R, T, U> TupleTransform<F, R> for (T, U)
where
    F: FnOnce(Self) -> R,
{
    fn transform(self, f: F) -> R {
        f(self)
    }
}

impl<F, R, T, U, V> TupleTransform<F, R> for (T, U, V)
where
    F: FnOnce(Self) -> R,
{
    fn transform(self, f: F) -> R {
        f(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_transform() {
        let (a, b) = (1, 2).transform(|(x, y)| (y, x));
        assert_eq!((a, b), (2, 1));

        let (a, b, c) = (1i64, 2, 3).transform(|(x, y, z)| (z, y, x));
        assert_eq!((a, b, c), (3, 2, 1));

        let (a, b, c) = (1i64, 2i32, "sdf").transform(|(x, y, z)| (z, x, y));
        assert_eq!((a, b, c), ("sdf", 1, 2));
    }
}
