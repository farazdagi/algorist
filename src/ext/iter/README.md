If you need to work with an iterator of items, one chunk of a given size at a time, you can use the
[`Chunks`](crate::ext::iter::chunks::Chunks) iterator.

If you need to fold an iterator while allowing for early termination, you can use the
[`FoldWhileExt::fold_while`](crate::ext::iter::fold_while::FoldWhileExt::fold_while) method.

When you need to work with pairs of consecutive items from an iterator, you can use the
[`SlidingWindowExt::sliding_window`](crate::ext::iter::window::SlidingWindowExt::sliding_window)
trait.
