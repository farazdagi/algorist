#![allow(clippy::needless_doctest_main)]

//! This module features utilities for reading input using the [`Scanner`], and
//! writing output using the [`Writer`] and [`macro@wln`] macro.
//!
//! # Examples
//!
//! You will typically use the [`test_cases()`] or [`test_case()`] functions, to
//! obtain a [`Scanner`] and a [`Writer`] when solving contest problems.
//!
//! ## Reading input of a single test
//!
//! It is less common nowadays, as most contest will provide you with several
//! test cases, but you can still do it, if you need to:
//!
//! ``` bash
//! # Input:
//! 2 3
//!
//! # Output:
//! Sum: 5
//! ```
//!
//! ``` no_run
//! use algorist::io::{test_case, wln};
//!
//! fn main() {
//!     test_case(&mut |scan, w| {
//!         let (a, b): (i32, i32) = scan.pair();
//!         wln!(w, "Sum: {}", a + b);
//!     });
//! }
//! ```
//!
//! ## Reading number of test cases and processing them
//!
//! Normally, in contest programming, you will have several test cases to
//! process.
//!
//! So, the input will start with a single integer `t`, which is the number of
//! test cases, followed by `t` test case inputs.
//!
//! ``` bash
//! # Input:
//! 2
//! 3 2
//! 1 2
//!
//! # Output:
//! Sum: 5
//! Sum: 3
//! ```
//!
//! ``` no_run
//! use algorist::io::{test_cases, wln};
//!
//! fn main() {
//!     test_cases(&mut |scan, w| {
//!         let (a, b): (i32, i32) = scan.pair();
//!         wln!(w, "Sum: {}", a + b);
//!     });
//! }
//! ```
//!
//! As you can see, the difference between reading a single test case and
//! reading multiple test cases is minimal -- you just need to call different
//! function, with the same closure.

use std::{
    collections::VecDeque,
    io::{self, BufWriter, StdinLock, StdoutLock, Write, prelude::*},
};

/// A helper function to read multiple test cases from standard input, and write
/// output to standard output.
///
/// # Example
///
/// ``` no_run
/// use algorist::io::{test_cases, wln};
///
/// // `test_cases` will read a `t` value from input, and then call the
/// // provided closure `t` times, allowing you to process each test case.
/// test_cases(&mut |scan, w| {
///     // You can use `u2()` to read a pair of `usize` values.
///     let a = scan.u(); // Read a single `usize`
///     let b = scan.u(); // Read another `usize`
///
///     wln!(w, "Sum: {}", a + b); // Write the sum to output
/// });
/// ```
///
/// ``` bash
/// # Input:
/// 2
/// 3 2
/// 2 1
///
/// # Output:
/// Sum: 5
/// Sum: 3
/// ```
///
/// In case you want to read a single test case, use the [`test_case()`],
/// instead.
pub fn test_cases<F: FnMut(&mut Scanner<StdinLock>, &mut Writer<BufWriter<StdoutLock>>)>(
    f: &mut F,
) {
    let mut scan = Scanner::new(io::stdin().lock());
    let mut w = Writer::new(io::BufWriter::new(io::stdout().lock()));

    scan.test_cases(&mut |scan| {
        f(scan, &mut w);
    });
}

/// A helper function to read a single test case from standard input, and write
/// to standard output.
///
/// # Example
///
/// ``` no_run
/// use algorist::io::{test_case, wln};
///
/// test_case(&mut |scan, w| {
///     // You can use `u2()` to read a pair of `usize` values.
///     let a = scan.u(); // Read a single `usize`
///     let b = scan.u(); // Read another `usize`
///
///     wln!(w, "Sum: {}", a + b); // Write the sum to output
/// });
/// ```
///
/// ``` bash
/// # Input:
/// 3 2
///
/// # Output:
/// Sum: 5
/// ```
pub fn test_case<F: FnMut(&mut Scanner<StdinLock>, &mut Writer<BufWriter<StdoutLock>>)>(f: &mut F) {
    let mut scan = Scanner::new(io::stdin().lock());
    let mut w = Writer::new(io::BufWriter::new(io::stdout().lock()));
    f(&mut scan, &mut w);
}

/// A `Writer` is a wrapper around `BufWriter<W>` that provides a convenient
/// interface for writing formatted output, without requiring to import
/// `std::io::Write` by the client code. It is expected to be used with [`wln!`]
/// macro.
///
/// # Example
///
/// ```no_run
/// use {
///     algorist::io::{Writer, wln},
///     std::io,
/// };
///
/// let mut w = Writer::new(io::BufWriter::new(io::stdout().lock()));
/// wln!(w, "Hello, {}!", "world");
/// writeln!(w, "This is a test."); // `wln!` is shorter and more ergonomic
/// ```
pub struct Writer<W: Write>(BufWriter<W>);

impl<W: Write> Writer<W> {
    pub fn new(inner: W) -> Self {
        Self(BufWriter::new(inner))
    }

    /// Writes a formatted string to the underlying writer.
    pub fn write_fmt(&mut self, args: std::fmt::Arguments) {
        let _ = self.0.write_fmt(args);
    }

    /// Flushes the underlying writer, ensuring all buffered data is written
    /// out.
    pub fn flush(&mut self) {
        let _ = self.0.flush();
    }
}

/// Scanner reads buffered input and parses it into tokens.
///
/// The `Scanner` is designed to simplify reading input in competitive
/// programming. It allows you to read various types of data from standard input
/// efficiently. Moreover, it supports several most common operations like
/// reading several test cases, strings, and vectors.
///
/// ## Reading input of a single test
///
/// It is less common nowadays, as most contest will provide you with several
/// test cases, but you can still read a single test case's input, if you need
/// to:
///
/// ``` bash
/// # Input:
/// 1 2 3 4
/// ```
///
/// ``` no_run
/// use {algorist::io::Scanner, std::io};
///
/// let mut scan = Scanner::new(io::stdin().lock());
///
/// let a: i32 = scan.next(); // reads a single token and parses it as `i32`
/// let a: i32 = scan.i(); // same as above
/// ```
///
/// ## Reading input from several test cases
///
/// Normally, in contest programming, you will have several test cases to
/// process. So, the input will start with a single integer `t`, which is the
/// number of test cases, followed by `t` test inputs.
///
/// `Scanner` provides a convenient method
/// [`test_cases()`](Scanner::test_cases) to read and process multiple tests
/// more ergonomically.
///
/// ``` bash
/// # Input:
/// 1
/// 3
/// 1 2 3
/// ```
///
/// ``` no_run
/// use {
///     algorist::io::{Scanner, wln},
///     std::io::{self, Write},
/// };
///
/// // Initialize a `Scanner` for reading input and a `BufWriter` for output.
/// let mut scan = Scanner::new(io::stdin().lock());
/// let mut w = io::BufWriter::new(io::stdout().lock());
///
/// // Read multiple test cases and process them.
/// scan.test_cases(&mut |scan| {
///     // Each test case reads a `usize` into `n` and then a vector of `n` integers.
///     let n = scan.u();
///     let vals: Vec<i32> = scan.vec(n);
///
///     // Simple wrapper around `writeln!` for convenience.
///     wln!(w, "{}", vals.len());
/// });
/// ```
///
/// # Important Note
///
/// For even more ergonomic usage, rely on the stand-alone [`test_cases()`]
/// which will create a [`Scanner`] and a [`Writer`] for you, and pass them to
/// the closure.
pub struct Scanner<R> {
    reader: R,
    buffer: Vec<u8>,
    iter: std::str::SplitWhitespace<'static>,
}

impl<R: BufRead> Scanner<R> {
    /// Creates a new `Scanner` instance with the given reader.
    ///
    /// # Example
    ///
    /// ``` no_run
    /// use {
    ///     algorist::io::Scanner,
    ///     std::io::{self, BufReader, Write},
    /// };
    ///
    /// // Read from standard input.
    /// let mut scan = Scanner::new(io::stdin().lock());
    ///
    /// // Write to standard output.
    /// let mut w = io::BufWriter::new(io::stdout().lock());
    ///
    /// let n: u16 = scan.next(); // Reads the next token as a `u16`.
    /// writeln!(w, "{}", n).unwrap(); // Writes the value to output.
    /// ```
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
            iter: "".split_whitespace(),
        }
    }

    /// Reads the next token from the input, parsing it into the specified `T`.
    ///
    /// This method will read until a newline character is encountered, then
    /// split the line into whitespace-separated tokens, and traverse the
    /// iterator.
    ///
    /// It will return the next token as type `T`.
    ///
    /// # Example
    ///
    /// ```
    /// use {algorist::io::Scanner, std::io::BufReader};
    ///
    /// let input = b"42 3.14 hello\n";
    /// let mut scan = Scanner::new(BufReader::new(input.as_ref()));
    /// let x: i32 = scan.next();
    /// let y: f64 = scan.next();
    /// let s: String = scan.next();
    /// assert_eq!(x, 42);
    /// assert_eq!(y, 3.14);
    /// assert_eq!(s, "hello");
    /// ```
    #[allow(clippy::should_implement_trait)]
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
                std::mem::transmute::<std::str::SplitWhitespace<'_>, std::str::SplitWhitespace<'_>>(
                    slice.split_whitespace(),
                )
            };
        }
    }

    /// Reads multiple test cases from the input, applying the provided function
    /// `f` to each test case.
    ///
    /// Normally, in contest problems, the first token read is the number of
    /// test cases `t`, and the function `f` is called `t` times, allowing
    /// you to process each test case individually.
    ///
    /// # Example
    ///
    /// ```
    /// use {algorist::io::Scanner, std::io::BufReader};
    ///
    /// let input = b"2\n1 2\n3 4\n";
    /// let mut scan = Scanner::new(BufReader::new(input.as_ref()));
    /// let mut sum = 0;
    /// scan.test_cases(&mut |scan| {
    ///     let x: i32 = scan.next();
    ///     let y: i32 = scan.next();
    ///     sum += x + y;
    /// });
    /// assert_eq!(sum, 10);
    /// ```
    pub fn test_cases<F: FnMut(&mut Self)>(&mut self, f: &mut F) {
        let t = self.u();
        for _ in 0..t {
            f(self);
        }
    }

    /// Reads the next token as a `usize`.
    ///
    /// # Example
    ///
    /// ```
    /// use {algorist::io::Scanner, std::io::BufReader};
    ///
    /// let input = b"42\n";
    /// let mut scan = Scanner::new(BufReader::new(input.as_ref()));
    /// let x: usize = scan.u();
    /// assert_eq!(x, 42);
    /// ```
    pub fn u(&mut self) -> usize {
        self.next()
    }

    /// Reads pair of `usize` values.
    ///
    /// # Example
    ///
    /// ```
    /// use {algorist::io::Scanner, std::io::BufReader};
    ///
    /// let input = b"42 43\n";
    /// let mut scan = Scanner::new(BufReader::new(input.as_ref()));
    /// let (x, y): (usize, usize) = scan.u2();
    /// assert_eq!(x, 42);
    /// assert_eq!(y, 43);
    /// ```
    pub fn u2(&mut self) -> (usize, usize) {
        (self.u(), self.u())
    }

    /// Reads triplet of `usize` values.
    ///
    /// See also [`u2`](Scanner::u2).
    pub fn u3(&mut self) -> (usize, usize, usize) {
        (self.u(), self.u(), self.u())
    }

    /// Reads quadruplet of `usize` values.
    ///
    /// See also [`u2`](Scanner::u2).
    pub fn u4(&mut self) -> (usize, usize, usize, usize) {
        (self.u(), self.u(), self.u(), self.u())
    }

    /// Reads the next token as an `i32`.
    pub fn i(&mut self) -> i32 {
        self.next()
    }

    /// Reads pair of `i32` values.
    pub fn i2(&mut self) -> (i32, i32) {
        (self.i(), self.i())
    }

    /// Reads triplet of `i32` values.
    pub fn i3(&mut self) -> (i32, i32, i32) {
        (self.i(), self.i(), self.i())
    }

    /// Reads quadruplet of `i32` values.
    pub fn i4(&mut self) -> (i32, i32, i32, i32) {
        (self.i(), self.i(), self.i(), self.i())
    }

    /// Reads pair of values of type `T`.
    ///
    /// # Example
    /// ```
    /// use {algorist::io::Scanner, std::io::BufReader};
    ///
    /// let input = b"1 2\n";
    /// let mut scan = Scanner::new(BufReader::new(input.as_ref()));
    /// let (x, y): (i32, i32) = scan.pair();
    /// assert_eq!(x + y, 3);
    ///
    /// let input = b"foo bar\n";
    /// let mut scan = Scanner::new(BufReader::new(input.as_ref()));
    /// let (x, y): (String, String) = scan.pair();
    /// assert_eq!(x, "foo");
    /// assert_eq!(y, "bar");
    /// ```
    pub fn pair<T: std::str::FromStr>(&mut self) -> (T, T) {
        (self.next(), self.next())
    }

    /// Reads triplet of values of type `T`.
    ///
    /// See also [`pair`](Scanner::pair).
    pub fn triplet<T: std::str::FromStr>(&mut self) -> (T, T, T) {
        (self.next(), self.next(), self.next())
    }

    /// Gets the next token as a `String`.
    pub fn string(&mut self) -> String {
        self.next()
    }

    /// Gets the next token as `Vec<u8>`.
    pub fn bytes(&mut self) -> Vec<u8> {
        self.string().bytes().collect()
    }

    /// Gets the next token as `Vec<char>`.
    pub fn chars(&mut self) -> Vec<char> {
        self.string().chars().collect()
    }

    /// Reads a vector of `T` from the input, where `n` is the number of
    /// elements.
    ///
    /// # Example
    /// ```
    /// use {algorist::io::Scanner, std::io::BufReader};
    ///
    /// let input = b"1 2 3\n";
    /// let mut scan = Scanner::new(BufReader::new(input.as_ref()));
    /// let v: Vec<i32> = scan.vec(3);
    /// assert_eq!(v, vec![1, 2, 3]);
    /// ```
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(n);
        (0..n).for_each(|_| result.push(self.next()));
        result
    }

    /// Reads a vector of `T` from the input, where `n` is the number of
    /// elements, and the first element is a default value for `T`.
    ///
    /// # Example
    /// ```
    /// use {algorist::io::Scanner, std::io::BufReader};
    ///
    /// let input = b"1 2 3\n";
    /// let mut scan = Scanner::new(BufReader::new(input.as_ref()));
    /// let v: Vec<i32> = scan.vec_padded(3);
    /// assert_eq!(v, vec![0, 1, 2, 3]);
    /// ```
    pub fn vec_padded<T: std::str::FromStr + Default>(&mut self, n: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(n + 1);
        result.push(T::default());
        (0..n).for_each(|_| result.push(self.next()));
        result
    }

    /// Reads a `VecDeque<T>` from the input, where `n` is the number of
    /// elements to read.
    ///
    /// # Example
    /// ```
    /// use {
    ///     algorist::io::Scanner,
    ///     std::{collections::VecDeque, io::BufReader},
    /// };
    ///
    /// let input = b"1 2 3\n";
    /// let mut scan = Scanner::new(BufReader::new(input.as_ref()));
    /// let v: VecDeque<i32> = scan.vec_deque(3);
    /// assert_eq!(v, VecDeque::from(vec![1, 2, 3]));
    /// ```
    pub fn vec_deque<T: std::str::FromStr>(&mut self, n: usize) -> VecDeque<T> {
        let mut result = VecDeque::with_capacity(n);
        (0..n).for_each(|_| result.push_back(self.next()));
        result
    }

    /// Reads a `HashSet<T>` from the input, where `n` is the number of
    /// elements to read.
    ///
    /// # Example
    /// ```
    /// use {
    ///     algorist::io::Scanner,
    ///     std::{collections::HashSet, io::BufReader},
    /// };
    ///
    /// let input = b"1 2 3\n";
    /// let mut scan = Scanner::new(BufReader::new(input.as_ref()));
    /// let set: HashSet<i32> = scan.hash_set(3);
    /// assert_eq!(set, HashSet::from([1, 2, 3]));
    /// ```
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
        v.iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ")
    )
    .unwrap();
}

/// A macro for writing a line with formatted output.
///
/// Just like `writeln!()`, but with a shorter name, and no return value (so,
/// now warning about unused result).
///
/// # Example
/// ```
/// use {
///     algorist::io::wln,
///     std::io::{self, Write},
/// };
///
/// let mut w = io::BufWriter::new(io::stdout().lock());
///
/// // Using more ergonomic `wln!` macro:
/// wln!(w, "Hello, {}!", "world");
///
/// // Alternatively, using the `writeln!()` macro directly:
/// let _ = writeln!(w, "Hello, {}!", "world");
/// ```
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
    use {super::*, crate::io::Scanner, std::io::BufReader};

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
