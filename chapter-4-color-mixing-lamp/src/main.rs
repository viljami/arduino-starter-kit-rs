#![no_std]
#![no_main]

use arduino_hal::simple_pwm::{IntoPwmPin, Prescaler, Timer1Pwm, Timer2Pwm};
use components::{print, println};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
    let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);

    let mut green_led = pins.d9.into_output().into_pwm(&timer1);
    let mut red_led = pins.d10.into_output().into_pwm(&timer1);
    let mut blue_led = pins.d11.into_output().into_pwm(&timer2);

    let red_sensor = pins.a0.into_analog_input(&mut adc);
    let green_sensor = pins.a1.into_analog_input(&mut adc);
    let blue_sensor = pins.a2.into_analog_input(&mut adc);

    print::init(arduino_hal::default_serial!(
        dp,
        pins,
        print::COMMUNICATION_SPEED
    ));

    green_led.enable();
    red_led.enable();
    blue_led.enable();

    loop {
        let red_sensor_value = red_sensor.analog_read(&mut adc);
        let green_sensor_value = green_sensor.analog_read(&mut adc);
        let blue_sensor_value = blue_sensor.analog_read(&mut adc);

        // println!(
        //     "Sensor rgb({}, {}, {})",
        //     red_sensor_value, green_sensor_value, blue_sensor_value
        // );

        let red = red_sensor_value / 4;
        let green = green_sensor_value / 4;
        let blue = blue_sensor_value / 4;

        println!("Color rgb({}, {}, {})", red, green, blue);

        green_led.set_duty(green as u8);
        red_led.set_duty(red as u8);
        blue_led.set_duty(blue as u8);

        arduino_hal::delay_ms(100);
    }
}
