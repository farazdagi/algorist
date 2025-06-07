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

## Installation

The crate provides cargo sub-command `algorist`, which can be installed using:

``` bash
cargo install algorist
```

NB: The crate is also a library of algorithms and data structures, which will be copied into your
contest project (see Usage section). Check the [documentation](https://docs.rs/algorist) for more
details.

## Usage

Basic usage pattern:

- Create new contest project (which is a normal Rust project, with some additional files)
- Work on a given problem, using conventional Cargo machinery to run/test
- Once happy, bundle the project into a single output file, which is expected by the contest system

### Create a new contest project

To create a new contest project:

``` bash
cargo algorist new <contest_id>
```

(`contest_id` will be normally contest number)

This will create Rust project in `contest-<contest_id>` directory with all the necessary problem and
algorithm modules copied into it.

### Work on a problem

All problems are located in `src/bin/<problem_id>.rs` file, where `<problem_id>` is one of `a..h`.

To test a problem, you can use:

``` bash
cargo test --bin <problem_id>
```

See the [documentation](https://docs.rs/algorist) on `io` module for more details on the default
code provided in problem files.

### Bundle the project

Contest systems expect a single output file, which is normally a binary executable. To bundle the
problem which you are working on, and which might include various additional modules (at the very
least `io` module is included), in a single output file, you can use:

``` bash
cargo algorist bundle <problem_id>
```

This will create a single output file in `bundled/<problem_id>.rs` file, which can be submitted to
the contest system.

## License

MIT
