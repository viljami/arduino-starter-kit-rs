#![no_std]
#![no_main]

use components::{print, println};
use panic_halt as _;

const ANGLE_MAX: u16 = 180;
const POTENTIOMETER_MAX: u16 = 1023;
const RANGE_MAX: u16 = 387;
const RANGE_MIN: u16 = 207;
const SERVO_COUNTS: u16 = 5000;
const SERVO_MAX: u16 = 580;
const SERVO_MIN: u16 = 135;

fn potentiometer_to_angle(potentiometer_value: u16) -> u16 {
    // Multiplier 10 for increased accuracy and added 1 for round up
    let a = potentiometer_value * 10 / (POTENTIOMETER_MAX * 10 / ANGLE_MAX + 1);
    if a > ANGLE_MAX {
        ANGLE_MAX
    } else {
        a
    }
}

fn angle_to_servo_ticks(angle: u16) -> u16 {
    // Multiplier 10 for increased accuracy and added 1 for round up
    let divider = (SERVO_MAX - SERVO_MIN) * 10 / ANGLE_MAX + 1;
    let a = (SERVO_MIN * 10 + angle * divider) / 10;
    if a > SERVO_MAX {
        SERVO_MAX
    } else {
        a
    }
}

fn adjust_range(servo_value: u16) -> u16 {
    let servo_value = RANGE_MIN - SERVO_MIN + servo_value;
    if servo_value > RANGE_MAX {
        RANGE_MAX
    } else {
        servo_value
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let potentiometer_pin = pins.a0.into_analog_input(&mut adc);
    pins.d9.into_output(); // Servo pin

    // - TC1 runs off a 250kHz clock, with 5000 counts per overflow => 50 Hz signal.
    // - Each count increases the duty-cycle by 4us.
    // - Use OC1A which is connected to D9 of the Arduino Uno.
    // Based on: https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-manual-servo.rs
    dp.TC1.icr1.write(|w| w.bits(SERVO_COUNTS));
    dp.TC1
        .tccr1a
        .write(|w| w.wgm1().bits(0b10).com1a().match_clear());
    dp.TC1
        .tccr1b
        .write(|w| w.wgm1().bits(0b11).cs1().prescale_64());

    print::init(arduino_hal::default_serial!(
        dp,
        pins,
        print::COMMUNICATION_SPEED
    ));

    loop {
        // Full range of motion
        let potentiometer_value = potentiometer_pin.analog_read(&mut adc);
        let angle = potentiometer_to_angle(potentiometer_value);
        let servo_value = angle_to_servo_ticks(angle);
        println!(
            "Potentiometer {} -> degrees {} -> servo {}",
            potentiometer_value, angle, servo_value
        );

        // Range of motion adjusted to the product interface
        dp.TC1.ocr1a.write(|w| w.bits(adjust_range(servo_value)));
        arduino_hal::delay_ms(500);
    }
}
