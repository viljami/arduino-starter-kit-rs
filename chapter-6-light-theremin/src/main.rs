#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::{
    clock::MHz16,
    hal::{
        port::{PB5, PC0},
        Adc,
    },
    port::{
        mode::{Analog, Output},
        Pin,
    },
};
use components::{print, println};
use millis::millis;
use panic_halt as _;

mod millis;

const PITCH_LOW: u32 = 50;
const PITCH_HIGH: u32 = 4000;
// const SERVO_COUNTS: u16 = 5000;

struct LightRange {
    low: u32,
    high: u32,
}

impl LightRange {
    fn new() -> Self {
        Self { low: 1023, high: 0 }
    }

    fn map(&self, value: u32, low: u32, high: u32) -> u32 {
        (value - self.low) * 1000 / (self.high - self.low) * (high - low) + low
    }
}

fn setup(
    led: &mut Pin<Output, PB5>,
    light_sensor: &Pin<Analog, PC0>,
    adc: &mut Adc<MHz16>,
) -> LightRange {
    let mut light_range = LightRange::new();

    led.set_high();

    while millis() < 5000 {
        let light_value = light_sensor.analog_read(adc) as u32;
        light_range.low = light_range.low.min(light_value);
        light_range.high = light_range.high.max(light_value);
    }

    led.set_low();

    println!("Range ({}, {})", light_range.low, light_range.high);

    light_range
}

// fn setup_piezo(dp: &Peripherals) {

// - TC1 runs off a 250kHz clock, with 5000 counts per overflow => 50 Hz signal.
// - Each count increases the duty-cycle by 4us.
// - Use OC1A which is connected to D9 of the Arduino Uno.
// Based on: https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-manual-servo.rs
// dp.TC1.icr1.write(|w| w.bits(SERVO_COUNTS));
// dp.TC1
//     .tccr1a
//     .write(|w| w.wgm1().bits(0b10).com1a().match_clear());
// dp.TC1
//     .tccr1b
//     .write(|w| w.wgm1().bits(0b11).cs1().prescale_64());
// }

// https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-simple-pwm-embedded-hal.rs
// fn fade(led: &mut impl SetDutyCycle, delay: &mut impl DelayNs) -> ! {
//     loop {
//         for pct in (0..=100).chain((0..100).rev()) {
//             led.set_duty_cycle_percent(pct).unwrap();
//             delay.delay_ms(10);
//         }
//     }
// }

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let pins = arduino_hal::pins!(dp);
    let light_sensor = pins.a0.into_analog_input(&mut adc);
    let mut led = pins.d13.into_output();
    let mut piezo = pins.d8.into_output();

    print::init(arduino_hal::default_serial!(
        dp,
        pins,
        print::COMMUNICATION_SPEED
    ));

    millis::init(dp.TC0);

    let light_range = setup(&mut led, &light_sensor, &mut adc);
    loop {
        let light = light_sensor.analog_read(&mut adc) as u32;
        let pitch = light_range.map(light, PITCH_LOW, PITCH_HIGH);
        let time_step = 1_000_000_000 / pitch;

        let start_m = millis();
        let mut m = start_m;
        while start_m - m < 1000 {
            piezo.toggle();
            arduino_hal::delay_us(time_step);
            m = millis();
        }

        println!("(light, pitch, time_step, dt): ({}, {}, {}, {}, {})", light, pitch, time_step, start_m - m, millis());
        // arduino_hal::delay_ms(10);
    }
}
