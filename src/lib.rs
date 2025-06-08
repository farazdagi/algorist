#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

/// Module for I/O operations.
///
/// This module features utilities for reading input using the
/// [`Scanner`](io::Scanner), and writing output using the [`wln!`](io::wln)
/// macro.
///
/// # Examples
///
/// The `Scanner` is designed to simplify reading input in competitive
/// programming. It allows you to read various types of data from standard input
/// efficiently. Moreover, it supports several most common operations like
/// reading several test cases, strings, and vectors.
///
/// ## Reading a single token
///
/// It is less common nowadays, as most contest will provide you with several
/// test cases, but you can still read a single token using the `Scanner`, if
/// you need to:
///
/// ``` bash
/// # Input:
/// 1 2 3 4
/// ```
///
/// ``` no_run
/// use std::io;
///
/// use algorist::io::{Scanner, wln};
///
/// let mut scan = Scanner::new(io::stdin().lock());
///
/// let a: i32 = scan.next(); // reads a single token and parses it as `i32`
/// let a: i32 = scan.i(); // same as above
///
/// let b: usize = scan.next();
/// let b: usize = scan.u();
/// ```
///
/// ## Reading number of test cases and processing them
///
/// Normally, in contest programming, you will have several test cases to
/// process. So, the input will start with a single integer `t`, which is the
/// number of test cases, followed by `t` test inputs.
///
/// `Scanner` provides a convenient method `test_cases()` to read and process
/// multiple tests more ergonomically.
///
/// ``` bash
/// # Input:
/// 1
/// 3
/// 1 2 3
/// ```
///
/// ``` no_run
/// use std::io::{self, Write};
///
/// use algorist::io::{Scanner, wln};
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
pub mod io;

/// Collections.
pub mod collections;

/// Various extensions to the standard library.
pub mod ext;

/// Mathematical utilities.
pub mod math;

/// Miscellaneous utilities.
pub mod misc;
