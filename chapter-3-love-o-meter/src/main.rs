#![no_std]
#![no_main]

use embedded_hal::digital::v2::OutputPin;
use panic_halt as _;
use ufmt::derive::uDebug;

#[macro_use]
mod print;

enum Pin<P2: OutputPin, P3: OutputPin, P4: OutputPin> {
    D2(P2),
    D3(P3),
    D4(P4),
}

#[derive(uDebug)]
struct PinError;

impl PinError {
    fn new<T>(_: T) -> Self {
        PinError
    }
}

impl<P2: OutputPin, P3: OutputPin, P4: OutputPin> Pin<P2, P3, P4> {
    fn off(&mut self) {
        let result = match self {
            Pin::D2(d) => d.set_low().map_err(PinError::new),
            Pin::D3(d) => d.set_low().map_err(PinError::new),
            Pin::D4(d) => d.set_low().map_err(PinError::new),
        };

        if let Err(err) = result {
            println!("Pin off error: {:?}", err)
        }
    }

    fn on(&mut self) {
        let result = match self {
            Pin::D2(d) => d.set_high().map_err(PinError::new),
            Pin::D3(d) => d.set_high().map_err(PinError::new),
            Pin::D4(d) => d.set_high().map_err(PinError::new),
        };

        if let Err(err) = result {
            println!("Pin on error: {:?}", err)
        }
    }
}

fn adc_to_micro_voltage(adc_value: u16) -> u64 {
    adc_value as u64 * 5 * 1_000_000 / 1024
}

fn micro_voltage_to_temperature(micro_voltage: u64) -> u64 {
    let a = micro_voltage * 100;
    let a = if a < 50_000_000 { 0 } else { a - 50_000_000 };
    a / 1_000_000
}

fn adc_to_temperature(adc_value: u16) -> u64 {
    micro_voltage_to_temperature(adc_to_micro_voltage(adc_value))
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let mut reassuring_led = pins.d13.into_output();
    let temperature_pin = pins.a0.into_analog_input(&mut adc);
    let mut love_leds = [
        Pin::D2(pins.d2.into_output()),
        Pin::D3(pins.d3.into_output()),
        Pin::D4(pins.d4.into_output()),
    ];

    print::init(arduino_hal::default_serial!(
        dp,
        pins,
        print::COMMUNICATION_SPEED
    ));

    love_leds.iter_mut().for_each(|led| led.off());

    let mut temperatures: [u64; 8] = [0; 8];

    for led_on_count in 1..9 {
        for (index, led) in love_leds.iter_mut().enumerate() {
            if index < led_on_count % 4 {
                led.on()
            } else {
                led.off()
            }
        }

        let temperature = adc_to_temperature(temperature_pin.analog_read(&mut adc));
        temperatures[led_on_count - 1] = temperature;

        arduino_hal::delay_ms(500);
    }

    arduino_hal::delay_ms(500);

    let room_temperature_average: u64 = temperatures
        .into_iter()
        .reduce(|a, b| a + b)
        .unwrap_or_default()
        / 8;

    loop {
        reassuring_led.toggle();
        let temperature = adc_to_temperature(temperature_pin.analog_read(&mut adc));
        println!("Temperature: {}", temperature);

        let led_on_count = match temperature {
            n if n < room_temperature_average + 3 => 0,
            n if n < room_temperature_average + 5 => 1,
            n if n < room_temperature_average + 7 => 2,
            _ => 3,
        };

        for (index, led) in love_leds.iter_mut().enumerate() {
            if index < led_on_count {
                led.on()
            } else {
                led.off()
            }
        }

        arduino_hal::delay_ms(200);
    }
}
