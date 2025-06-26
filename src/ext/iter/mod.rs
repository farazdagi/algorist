//! Extensions to the standard library's `Iterator` trait.
//!
//! If you need to work with an iterator of items, one chunk of a given size at
//! a time, you can use the [`Chunks`][chunks::Chunks] iterator.
//!
//! If you need to fold an iterator while allowing for early termination, you
//! can use the
//! [`FoldWhileExt::fold_while`][fold_while::FoldWhileExt::fold_while]
//! method.

pub mod chunks;
pub mod fold_while;
pub mod window;
