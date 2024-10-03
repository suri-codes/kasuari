# Kasuari

A Rust implementation of the Cassowary constraint solving algorithm ([Badros et. al 2001]). It is
based heavily on the implementation the C++ [Kiwi] library. The implementation does however differ
in some details.

This library is a fork of [Cassowary-rs], by Dylan Ede, which hasn't been maintained since 2018.
`Kasuari` is the Indonesian name for the Cassowary bird.

Cassowary is designed for solving constraints to lay out user interfaces. Constraints typically take
the form "this button must line up with this text box", or "this box should try to be 3 times the
size of this other box". Its most popular incarnation by far is in Apple's Autolayout system for Mac
OS X and iOS user interfaces. UI libraries using the Cassowary algorithm manage to achieve a much
more natural approach to specifying UI layouts than traditional approaches like those found in HTML.

This library is a low level interface to the solving algorithm, though it tries to be as convenient
as possible. As a result it does not have any intrinsic knowledge of common user interface
conventions like rectangular regions or even two dimensions. These abstractions belong in a higher
level crate.

For more information, please read the [Kasuari API docs].

## Getting Started

Add this crate to your Cargo.toml file

```shell
cargo add kasuari
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE)
- MIT license ([LICENSE-MIT](./LICENSE-MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

[Badros et. al 2001]: https://constraints.cs.washington.edu/solvers/cassowary-tochi.pdf
[Kiwi]: https://github.com/nucleic/kiwi
[Cassowary-rs]: https://crates.io/crates/cassowary
[Kasuari API docs]: https://docs.rs/kasuari
