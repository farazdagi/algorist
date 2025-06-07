# algorist

[![crates.io](https://img.shields.io/crates/d/algorist.svg)](https://crates.io/crates/algorist)
[![docs.rs](https://docs.rs/algorist/badge.svg)](https://docs.rs/algorist)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![dependencies](https://deps.rs/repo/github/farazdagi/algorist/status.svg)](https://deps.rs/repo/github/farazdagi/algorist)

Helper tools, algorithms and data structures for competitive programming.

## Motivation

To provide a collection of algorithms and data structures that are commonly used in competitive
programming, with necessary helper tools to test and bundle files into single output file -- which
is expected in competitive programming.

## Installation & Usage

The crate provides cargo sub-command `algorist`, which can be installed using:

``` bash
cargo install algorist
```

To create a new contest project:

``` bash
cargo algorist new <contest_id>
```

This will create Rust project in `contest-<contest_id>` directory with all the necessary problem and
algorithm modules copied into it.

## License

MIT
