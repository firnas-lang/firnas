# The Firnas Programming Language

This is the main source code for [Firnas](https://firnas-lang.dev). It contains the compiler and the standard library.

## Running Firnas

### Playground

You can use Firnas in your browser using WebAssembly. [Firnas Playground](https://play.firnas-lang.dev)

### Locally

We currently haven't made any pre-built binaries yet. To install locally first you need to install [Rust using `rustup`](https://www.rust-lang.org/tools/install), then run:

```bash
cargo install --locked firnas
```

```bash
firnas compile ./example
```

## License

Firnas is is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](./LICENCE-APACHE) and [LICENSE-MIT](./LICENCE-MIT).
