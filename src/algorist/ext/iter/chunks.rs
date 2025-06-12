pub struct Chunks<I>
where
    I: Iterator,
{
    iter: I,
    chunk_size: usize,
}

impl<I> Chunks<I>
where
    I: Iterator,
{
    pub fn new(iter: I, chunk_size: usize) -> Self {
        Self { iter, chunk_size }
    }
}

impl<I> Iterator for Chunks<I>
where
    I: Iterator,
{
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
