# Chapter 5: Mood Cue

Rust project for the _Arduino Uno_.

My servo motor cables were in red, white, black order which made me need to change the order of the wiring on the board.

I am starting to miss red and black cables for power and ground connections. It would make the connections a lot easier to reason. I'd like to the power and ground from the board analog and digital inputs and outputs - all with different colors. And be consistent. Right now I am happy to be experiencing the need for color difference. I can leverage it after the starter kit exercises.

The header pins for the female connector of the servo motor were not plug and play. There is a black center piece for the pins but it was too far on the other end of the pins. It is possible to move the black piece more to the middle that the connection worked for the breadboard. This must sound to anyone with experience like a no-brainer but it took some courage to try it out without googling first.

Potentiometer is a huge component compared to the one on the blueprint. That too changed the implementation on the breadboard.

I copied the implementation for servo_pin from an arv-hal Arduino Uno [example](https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-manual-servo.rs). Without the example I would not have been able to do it... yet.

Electronics is such a thrill.

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
