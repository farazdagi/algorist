# algorist

Helper tools, algorithms and data structures for competitive programming.

## Usage

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
The file will contain entry point `main` function, which is expected to read input from standard
input and write output to standard output.

To test a problem, you can use:

``` bash
cargo test --bin <problem_id>
```

See the [`documentation`](https://docs.rs/algorist/latest/algorist/io/) on `io` module for more
details on the default code provided in problem files.

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

Included modules:

- [`io`](https://docs.rs/algorist/latest/algorist/io/) - input/output helpers, including `Scanner`
  for reading input and `wln!` macro for writing output.

## License

MIT
