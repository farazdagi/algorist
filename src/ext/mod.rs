//! Extensions to the standard library types.
//!
//! # Iterators
//!
//! The [`iter`] module provides extensions to iterators.
//!
//! If you need to work with an iterator of items, one chunk of a given size at
//! a time, you can use the [`Chunks`][iter::chunks::Chunks] iterator.
//!
//! If you need to fold an iterator while allowing for early termination, you can use the [`FoldWhileExt::fold_while`][iter::fold_while::FoldWhileExt::fold_while]
//! method.

pub mod iter;
pub mod slice;
pub mod tuple;
pub mod vec;
