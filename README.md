# Arduino Starter Kit in Rust

Arduino Starter Kit exercises written in Rust. The target board is the [_Arduino Uno R3_](https://store.arduino.cc/products/arduino-uno-rev3). To setup the development environment for Rust see [Chapter 1: Setting Up](chapter-1-setting-up/README.md).

Shared resources for chapters are in folder [components](components/README.md).

## Chapters

1. [Setting Up](chapter-1-setting-up/README.md)
2. [Spaceship Interface](chapter-2-spaceship-interface/README.md)
3. [Love-O-Meter](chapter-3-love-o-meter/README.md)
4. [Color Mixing Lamp](chapter-4-color-mixing-lamp/README.md)
5. [Mood Cue](chapter-5-mood-cue/README.md)
6. [Light Theremin](chapter-6-light-theremin/README.md)

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
* [Design Contracts](https://docs.rust-embedded.org/book/static-guarantees/design-contracts.html)
* [Zero cost abstraction](https://docs.rust-embedded.org/book/static-guarantees/zero-cost-abstractions.html).

## License

[MIT LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>
