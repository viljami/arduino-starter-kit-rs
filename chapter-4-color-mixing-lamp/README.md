# Chapter 4: Color Mixing Lamp

Rust project for the _Arduino Uno_.

I needed to place the phototransistors more apart than in the chapter blueprint. This created the space to be able to use the color filters.

Normally I don't like leaving commented out code in the repo... maybe the path of my stumble to get the exercise working is better documented with that particular commented out println.

## Build Instructions

1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude
