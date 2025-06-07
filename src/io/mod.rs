use std::collections::VecDeque;
use std::io::prelude::*;

/// Scanner reads buffered input and parses it into tokens.
pub struct Scanner<R> {
    reader: R,
    buffer: Vec<u8>,
    iter: std::str::SplitWhitespace<'static>,
}

impl<R: BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
            iter: "".split_whitespace(),
        }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buffer.clear();
            self.reader
                .read_until(0xA, &mut self.buffer)
                .expect("Failed read");

            self.iter = unsafe {
                let slice = std::str::from_utf8_unchecked(&self.buffer);
                std::mem::transmute(slice.split_whitespace())
            };
        }
    }

    pub fn test_cases<F: FnMut(&mut Self)>(&mut self, f: &mut F) {
        let t = self.u();
        for _ in 0..t {
            f(self);
        }
    }

    pub fn u(&mut self) -> usize {
        self.next()
    }

    pub fn u2(&mut self) -> (usize, usize) {
        (self.u(), self.u())
    }

    pub fn u3(&mut self) -> (usize, usize, usize) {
        (self.u(), self.u(), self.u())
    }

    pub fn u4(&mut self) -> (usize, usize, usize, usize) {
        (self.u(), self.u(), self.u(), self.u())
    }

    pub fn i(&mut self) -> i32 {
        self.next()
    }

    pub fn i2(&mut self) -> (i32, i32) {
        (self.i(), self.i())
    }

    pub fn i3(&mut self) -> (i32, i32, i32) {
        (self.i(), self.i(), self.i())
    }

    pub fn i4(&mut self) -> (i32, i32, i32, i32) {
        (self.i(), self.i(), self.i(), self.i())
    }

    pub fn pair<T: std::str::FromStr>(&mut self) -> (T, T) {
        (self.next(), self.next())
    }

    pub fn triplet<T: std::str::FromStr>(&mut self) -> (T, T, T) {
        (self.next(), self.next(), self.next())
    }

    pub fn string(&mut self) -> String {
        self.next()
    }

    pub fn bytes(&mut self) -> Vec<u8> {
        self.string().bytes().collect()
    }

    pub fn chars(&mut self) -> Vec<char> {
        self.string().chars().collect()
    }

    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(n);
        (0..n).for_each(|_| result.push(self.next()));
        result
    }

    pub fn vec_padded<T: std::str::FromStr + Default>(&mut self, n: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(n + 1);
        result.push(T::default());
        (0..n).for_each(|_| result.push(self.next()));
        result
    }

    pub fn vec_deque<T: std::str::FromStr>(&mut self, n: usize) -> VecDeque<T> {
        let mut result = VecDeque::with_capacity(n);
        (0..n).for_each(|_| result.push_back(self.next()));
        result
    }

    pub fn hash_set<T: std::hash::Hash + std::cmp::Eq + std::str::FromStr>(
        &mut self,
        n: usize,
    ) -> std::collections::HashSet<T> {
        let mut result = std::collections::HashSet::with_capacity(n);
        (0..n).for_each(|_| {
            result.insert(self.next());
        });
        result
    }
}

fn wv<W: Write, T: std::fmt::Display>(w: &mut W, v: &[T]) {
    write!(
        w,
        "{}",
        v.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
    .unwrap();
}

#[macro_export]
macro_rules! wln_impl {
    ($($es:expr),+) => {{
        let _ = writeln!($($es),+);
    }}
}
pub use wln_impl as wln;

pub fn wvln<W: Write, T: std::fmt::Display>(w: &mut W, v: &[T]) {
    wv(w, v);
    writeln!(w).ok();
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::*;
    use crate::io::Scanner;

    #[test]
    fn read_test_cases() {
        let input = b"2\n1 2\n3 4\n";
        let mut scanner = Scanner::new(BufReader::new(input.as_ref()));
        let mut sum = 0;
        scanner.test_cases(&mut |scanner| {
            let x: i32 = scanner.next();
            let y: i32 = scanner.next();
            sum += x + y;
        });
        assert_eq!(sum, 10);
    }

    #[test]
    fn read_i32_list() {
        let input = b"1 2\n";
        let mut scanner = Scanner::new(BufReader::new(input.as_ref()));

        let x: i32 = scanner.next();
        let y: i32 = scanner.next();
        assert_eq!(x + y, 3);
    }

    #[test]
    fn read_usize_list() {
        let input = b"1 2\n";
        let mut scanner = Scanner::new(BufReader::new(input.as_ref()));
        let (x, y) = scanner.u2();
        assert_eq!(x + y, 3);

        let input = b"1 2 3\n";
        let mut scanner = Scanner::new(BufReader::new(input.as_ref()));
        let (x, y, z) = scanner.u3();
        assert_eq!(x + y + z, 6);
    }

    #[test]
    fn read_pair() {
        let input = b"1 2\n";
        let mut scanner = Scanner::new(BufReader::new(input.as_ref()));
        let (x, y): (i32, i32) = scanner.pair();
        assert_eq!(x + y, 3);
    }

    #[test]
    fn read_triplet() {
        let input = b"1 2 3\n";
        let mut scanner = Scanner::new(BufReader::new(input.as_ref()));
        let (x, y, z): (i32, i32, i32) = scanner.triplet();
        assert_eq!(x + y + z, 6);
    }

    #[test]
    fn read_string() {
        let input = b"hello\nworld\n";
        let mut scanner = Scanner::new(BufReader::new(input.as_ref()));
        let s: String = scanner.string();
        assert_eq!(s, "hello");
        let s: String = scanner.string();
        assert_eq!(s, "world");
    }

    #[test]
    fn read_byte_list() {
        let input = b"abc\n";
        let mut scanner = Scanner::new(BufReader::new(input.as_ref()));
        let bytes: Vec<u8> = scanner.bytes();
        assert_eq!(bytes, vec![b'a', b'b', b'c']);
    }

    #[test]
    fn read_char_list() {
        let input = b"abc\n";
        let mut scanner = Scanner::new(BufReader::new(input.as_ref()));
        let chars: Vec<char> = scanner.chars();
        assert_eq!(chars, vec!['a', 'b', 'c']);
    }

    #[test]
    fn read_vec() {
        let input = b"1 2 3\n";
        let mut scanner = Scanner::new(BufReader::new(input.as_ref()));
        let v: Vec<i32> = scanner.vec(3);
        assert_eq!(v, vec![1, 2, 3]);

        let input = b"1 2 3\n";
        let mut scanner = Scanner::new(BufReader::new(input.as_ref()));
        let v: Vec<i32> = scanner.vec_padded(3);
        assert_eq!(v, vec![0, 1, 2, 3]);
    }

    #[test]
    fn write_vec() {
        let mut output = Vec::new();
        wv(&mut output, &vec![1, 2, 3]);
        assert_eq!(output, b"1 2 3");
    }
}
