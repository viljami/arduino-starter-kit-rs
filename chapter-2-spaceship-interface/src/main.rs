#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut reassuring_led = pins.d13.into_output();

    let button = pins.d2.into_floating_input();
    let mut green = pins.d3.into_output();
    let mut red_1 = pins.d4.into_output();
    let mut red_2 = pins.d5.into_output();

    loop {
        reassuring_led.toggle();

        if button.is_low() {
            green.set_high();
            red_1.set_low();
            red_2.set_low();
        } else {
            green.set_low();
            red_1.set_low();
            red_2.set_high();

            arduino_hal::delay_ms(250);

            red_1.set_high();
            red_2.set_low();

            arduino_hal::delay_ms(250);
        }

        arduino_hal::delay_ms(500);
    }
}
