[Draft] chapter-6-light-theremin
========================

Rust project for the _Arduino Uno_ with piezo noise source.

The implementation is not yet tested to produce the exact required frequency.

![Arduino board with a piezo](https://github.com/viljami/arduino-starter-kit-rs/blob/main/assets/chapter-6.jpg?raw=true)

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
