/// Iterator extension to split an iterator into chunks of a specified size.
///
/// Given a source iterator and a chunk size, this iterator will yield vectors
/// of items, where each vector contains up to `chunk_size` items from the
/// source iterator. If the source iterator has fewer items than the specified
/// chunk size, the last vector will contain the remaining items.
///
/// # Example
///
/// ```
/// use algorist::ext::iter::chunks::Chunks;
///
/// let v = vec![1, 2, 3, 4, 5, 6, 7];
/// let chunks = Chunks::new(v.into_iter(), 3).collect::<Vec<_>>();
/// assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
/// ```
///
/// Normally, you would use the [`ChunksExt::chunks`] method on an iterator to
/// achieve the same result:
///
/// ```
/// use algorist::ext::iter::chunks::ChunksExt;
///
/// let v = vec![1, 2, 3, 4, 5, 6, 7];
/// let chunks = v.into_iter().chunks(3).collect::<Vec<_>>();
/// assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
/// ```
pub struct Chunks<I> {
    iter: I,
    chunk_size: usize,
}

impl<I> Chunks<I> {
    pub fn new(iter: I, chunk_size: usize) -> Self {
        Self { iter, chunk_size }
    }
}

impl<I: Iterator> Iterator for Chunks<I> {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = Vec::with_capacity(self.chunk_size);
        for _ in 0..self.chunk_size {
            if let Some(item) = self.iter.next() {
                chunk.push(item);
            } else {
                break;
            }
        }
        if chunk.is_empty() { None } else { Some(chunk) }
    }
}

/// Extension trait for iterators to provide a method for chunking.
///
/// This trait adds the `chunks` method to any iterator, allowing it to be split
/// into chunks of a specified size.
///
/// # Example
///
/// ```
/// use algorist::ext::iter::chunks::ChunksExt;
///
/// let v = vec![1, 2, 3, 4, 5, 6, 7];
/// let chunks = v.into_iter().chunks(3).collect::<Vec<_>>();
/// assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
/// ```
pub trait ChunksExt: Iterator {
    fn chunks(self, chunk_size: usize) -> Chunks<Self>
    where
        Self: Sized,
    {
        Chunks::new(self, chunk_size)
    }
}

impl<I: Iterator> ChunksExt for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunks() {
        let v = vec![1, 2, 3, 4, 5, 6, 7];
        let chunks = Chunks::new(v.into_iter(), 3).collect::<Vec<_>>();
        assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
    }

    #[test]
    fn chunks_ext() {
        let v = vec![1, 2, 3, 4, 5, 6, 7];
        let chunks = v.into_iter().chunks(3).collect::<Vec<_>>();
        assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5, 6], vec![7]]);
    }
}
