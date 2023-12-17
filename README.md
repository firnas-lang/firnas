# The Firnas Programming Language

This is the main source code for [Firnas](https://firnas-lang.dev). It contains the compiler and the standard library.

## Running Firnas

### Playground

You can use Firnas in your browser using WebAssembly. [Firnas Playground](https://play.firnas-lang.dev)

### Locally

We currently haven't made any pre-built binaries yet.

To install locally first you need:

- [Rust using `rustup`](https://www.rust-lang.org/tools/install)
- [git](https://git-scm.com/):

Now you can either install a specific version from our [GitHub releases](https://github.com/firnas-lang/firnas/releases), or install from master:

```bash
cargo install --locked --no-default-features --features ar --git https://github.com/firnas-lang/firnas.git firnasc

firnasc compile <path/to/file>
```

## License

Firnas is is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENCE-APACHE) and [LICENSE-MIT](./LICENCE-MIT).
