# Chapter 2 Spaceship Interface

Rust project for the _Arduino Uno_ with a button to trigger red leds and a green led to indicate idle mode.

![Arduino board with a button and](https://github.com/viljami/arduino-starter-kit-rs/blob/main/assets/chapter-2.gif?raw=true)


## Build Instructions

1. Install prerequisites as described in the [Chapter 1](../chapter-1-setting-up/README.md).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.
