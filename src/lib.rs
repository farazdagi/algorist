#![doc = include_str!("../README.md")]

/// Module for I/O operations.
///
/// This module features utilities for reading input using the
/// [`Scanner`](io::Scanner), and writing output using the [`wln!`](io::wln)
/// macro.
///
/// # Example
///
/// ``` no_run
/// use std::io::{self, Write};
///
/// use algorist::io::{Scanner, wln};
///
/// fn main() {
///     // Initialize a `Scanner` for reading input and a `BufWriter` for output.
///     let mut scan = Scanner::new(io::stdin().lock());
///     let mut w = io::BufWriter::new(io::stdout().lock());
///
///     // Read multiple test cases and process them.
///     scan.test_cases(&mut |scan| {
///         // Each test case reads a `usize` into `n` and then a vector of `n` integers.
///         let n = scan.u();
///         let vals: Vec<i32> = scan.vec(n);
///
///         // Simple wrapper around `writeln!` for convenience.
///         wln!(w, "{}", vals.len());
///     });
/// }
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
