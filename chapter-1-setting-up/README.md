# Chapter 1: Setting Up

Install [rustup](https://rustup.rs/) to manage Rust
build targets on your machine.

```sh
# Install nightly version on Rust
rustup install nightly

# Clippy helps to write clean Rust code
rustup component add clippy --toolchain nightly-2023-08-08-x86_64-apple-darwin
```

[Brew](https://brew.sh/) is handy package manager for OSX.

You can use any editor of your choice for the Rust code. I have used [Visual Studio Code](https://code.visualstudio.com/). I recommend installing Rust related extensions from the editor's 'Extensions' tab.

```sh
brew install vscode
```

## Setup Development environment

Setting up for [Linux and Windows](https://blog.logrocket.com/complete-guide-running-rust-arduino/). Here I have confirmed functional OSX setup:

```sh
xcode-select --install
brew tap osx-cross/avr
brew install avr-binutils avr-gcc avrdude
brew install lsusb # For finding the connected Arduino board
```

[`avr-hal`](https://github.com/Rahix/avr-hal#readme) is Hardware Abstraction Layer for AVR microcontrollers.

[Ravedude](https://crates.io/crates/ravedude) handles everything from finding the board, flashing the board, and listening to connections. Use with the `cargo run` command. To install Ravedude run the following command.

```sh
cargo +stable install ravedude
```

## Build Instructions

Connect your board to your computer with usb cable.

```sh
# Find Arduino board
# -v flag stands for Verbose output
# The flag helped to confirm that the Arduino Uno board is connected
lsusb -v
# Output:
# ...
# Product ID: 0x0043
# Vendor ID: 0x2341
# Version: 0.01
# Serial Number: 34239323339351311270
# Speed: Up to 12 Mb/s
# Manufacturer: Arduino (www.arduino.cc)
# ...

l /dev/tty.* # Arduino on OSX prints like tty.usbmodem146301
# Set ravedude envionment variable
export RAVEDUDE_PORT=/dev/tty.usbmodem146301
```

## Create your first project

```sh
cargo install cargo-generate
cargo generate --git <https://github.com/Rahix/avr-hal-template.git>
```

```sh
# Build the firmware
cargo build

# Flash the firmware to a connected board
cargo run
```

If `ravedude` fails to detect your board, check its documentation at <https://crates.io/crates/ravedude>.
`ravedude` will open a console session after flashing where you can interact with the UART console of your board.
