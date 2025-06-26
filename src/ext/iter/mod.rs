//! Extensions to the standard library's `Iterator` trait.
//!
//! If you need to work with an iterator of items, one chunk of a given size at
//! a time, you can use the [`Chunks`][chunks::Chunks] iterator.

pub mod chunks;
pub mod fold_while;
pub mod window;
