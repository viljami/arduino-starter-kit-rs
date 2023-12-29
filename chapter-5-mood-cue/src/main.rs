#![no_std]
#![no_main]

use components::{print, println};
use panic_halt as _;

fn potentiometer_to_angle(potentiometer_value: u16) -> u16 {
    let a = potentiometer_value * 10 / 57; // round_up(1024 / 180)
    if a > 180 {
        180
    } else {
        a
    }
}

fn angle_to_servo_ticks(angle: u16) -> u16 {
    let a = (1350 + angle * 25) / 10; // (580 - 135) / 180 = 2,45 -> 2
    if a > 580 {
        580
    } else {
        a
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
    let tc1 = dp.TC1;
    tc1.icr1.write(|w| w.bits(4999));
    tc1.tccr1a
        .write(|w| w.wgm1().bits(0b10).com1a().match_clear());
    tc1.tccr1b
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
        let servo_value = 207 - 135 + servo_value;
        let servo_value = if servo_value > 387 { 387 } else { servo_value };

        // Based on: https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-manual-servo.rs
        tc1.ocr1a.write(|w| w.bits(servo_value));
        arduino_hal::delay_ms(500);
    }
}
