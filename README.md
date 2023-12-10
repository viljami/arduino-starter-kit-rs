# Arduino Starter Kit in Rust

Arduino Starter Kit exercises written in Rust. The target board is the [_Arduino Uno R3_](https://store.arduino.cc/products/arduino-uno-rev3).

I am big on Rust and picked it for working with Arduino Starter Kit. I got interested using Rust due to [Design Contracts](https://docs.rust-embedded.org/book/static-guarantees/design-contracts.html) and related [Zero cost abstraction](https://docs.rust-embedded.org/book/static-guarantees/zero-cost-abstractions.html).

To setup the development environment for Rust see [Chapter 1: Setting Up](chapter-1-setting-up/README.md).

Chapters

1. [Setting Up](chapter-1-setting-up/README.md)
    * Includes instructions to setup the development environment
2. Spaceship Interface

## Debugging

Later on it might be useful to be able to profile the applications. The quick way is starting with `flamegraph` and reading the article: [Profiling Rust programs the easy way](https://ntietz.com/blog/profiling-rust-programs-the-easy-way/).

```sh
# Install
cargo install flamegraph

# Run
cargo flamegraph
```

## Other reading

* [Embedded Rust Book](https://docs.rust-embedded.org/book/)
* [Embedded Rust Weekly Newsletter](https://www.trackawesomelist.com/rust-embedded/awesome-embedded-rust/week/)
* [Embassy Framework on ESP](https://apollolabsblog.hashnode.dev/embassy-on-esp-getting-started)

## License

* MIT license
   ([LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>)
