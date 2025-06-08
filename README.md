# algorist

[![crates.io](https://img.shields.io/crates/d/algorist.svg)](https://crates.io/crates/algorist)
[![docs.rs](https://docs.rs/algorist/badge.svg)](https://docs.rs/algorist)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![dependencies](https://deps.rs/repo/github/farazdagi/algorist/status.svg)](https://deps.rs/repo/github/farazdagi/algorist)

Helper tools, algorithms and data structures for competitive programming.

Algorist is both a CLI tool for managing programming contest projects AND a collection of useful
algorithms and data structures to use in those projects.

## Installation

The crate provides cargo sub-command `algorist`, which can be installed using:

``` bash
cargo install algorist
```

Once installed, you can use it as `cargo algorist`.

NB: No point in installing the crate as a library (except for development of the crate itself).

## Usage

When contesting, you will normally have a set of problems to solve, each of which is identified by a
problem ID (usually a letter from `a` to `h`). Each problem will have its own source file, and while
that file can use any number of additional modules, it is expected that the final submission is a
single file that contains all the necessary code to solve the problem.

The `algorist` CLI tool provides a way to create a new contest project, which is a normal Rust
project, use additional modules with common algorithms and data structures, and then bundle each
problem into a single output file that can be submitted to the contest system (only modules actually
used will be bundled, not all available data structures and algorithms).

### Create a new contest project

To create a new contest project (`contest_id` will be normally contest number):

``` bash
cargo algorist new <contest_id>

# examples:
cargo algorist new 4545
cargo algorist new contests/4545 # sub-folders are also supported
```

This will create Rust project with all the necessary problem files and algorithm modules copied into
it.

The project structure will look something like this:

``` text
contest-4545
├── src
│   ├── lib.rs
│   ├── io
│   │   └── mod.rs
│   │   ... some additional modules (math, collections etc)
│   └── bin
│       ├── a.rs
│       ├── b.rs
│       ├── c.rs
│       ├── d.rs
│       ├── e.rs
│       ├── f.rs
│       ├── g.rs
│       └── h.rs
├── rustfmt.toml
├── Cargo.toml
└── Cargo.lock

```

If you don't want to have initial problem files, you can create a new contest project with `--empty`
flag:

``` bash
cargo algorist new <contest_id> --empty
```

Later, you can always add problems into `src/bin` directory using:

``` bash
cargo algorist add <problem_id>

# examples:
cargo algorist add a        # `.rs` is not required
cargo algorist add a.rs     # same as above
```

### Work on a problem

All problems are located in `src/bin/<problem_id>.rs` files. The file will contain entry point
`main` function, which is expected to read input from standard input and write output to standard
output:

``` rust, no_run
use std::io::{self, Write};
use algorist::io::{Scanner, wln};

fn main() {
    let mut scan = Scanner::new(io::stdin().lock());
    let mut w = io::BufWriter::new(io::stdout().lock());

    scan.test_cases(&mut |scan| {
        let n = scan.u();
        let vals: Vec<i32> = scan.vec(n);

        let ans = vals.len();
        wln!(w, "{}", ans);
    });
}

```

To test a problem, you can use (again, it is a normal Rust project, so you can use all the usual
machinery):

``` bash
cargo test --bin <problem_id>

# alias pbpaste=’xsel — clipboard — output’ on Linux
pbpaste | cargo run --bin <problem_id>   # run with input from clipboard
cargo run --bin <problem_id> < input.txt # run with input from file
```

NB: See the [`documentation`](https://docs.rs/algorist/latest/algorist/io/) on `io` module for more
details on the default code provided in problem files.

Once you are happy with your solution, you can submit it to the contest system (by bundling into a
single file).

To add new problem file into `src/bin` directory, use:

``` bash
cargo algorist add <problem_id>
```

### Bundle the project

Contest systems expect a single output file, where all used modules are packed within the scope of
that file.

To bundle the problem which you are working on, and which might include various additional modules
(at the very least `io` module is included), in a single output file, you can use:

``` bash
cargo algorist bundle <problem_id>
```

This will create a single output file in `bundled/<problem_id>.rs` file, which can be submitted to
the contest system.

Note that while the library provides a lot of algorithms and data structures, only those actually
used in the problem will be included in the output file, so the final file will be as concise and
readable as possible (it is NOT just a dump of everything).

## Included algorithms and data structures

The crate is also a library of algorithms and data structures, which will be copied into your
contest project, and can be used in your problem files.

| Module | Description |
| --- | --- |
| [`io`](https://docs.rs/algorist/latest/algorist/io/) | Input/output helpers, including `Scanner` for reading input and `wln!` macro for writing output. |

See [`Modules`](https://docs.rs/algorist/latest/algorist/#modules) section in the documentation for
a complete list of available modules.

## License

MIT
