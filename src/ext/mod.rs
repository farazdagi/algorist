//! Extensions to the standard library types.
//!
//! # Iterators
//!
//! The [`iter`] module provides extensions to iterators.
//!
//! If you need to work with an iterator of items, one chunk of a given size at
//! a time, you can use the [`Chunks`][iter::chunks::Chunks] iterator.

pub mod iter;
pub mod slice;
pub mod tuple;
pub mod vec;
