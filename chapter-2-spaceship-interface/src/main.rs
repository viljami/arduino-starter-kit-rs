#![no_std]
#![no_main]

use panic_halt as _;

type StepsLeft = u8;

struct Count {
    start: StepsLeft,
    current: StepsLeft,
}

impl Count {
    fn new(start: StepsLeft) -> Self {
        Count {
            start,
            current: start
        }
    }

    fn decrement(&self) -> Self {
        Count {
            start: self.start,
            current: self.current - 1,
        }
    }
}

enum State {
    Loading(Count),
    Idle,
    Phasers(Count),
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut reassuring_led = pins.d13.into_output();

    let button = pins.d2.into_floating_input();
    let mut green = pins.d3.into_output();
    let mut red_1 = pins.d4.into_output();
    let mut red_2 = pins.d5.into_output();

    let mut current_state = State::Loading(Count::new(120));

    loop {
        reassuring_led.toggle();

        current_state = match current_state {
            State::Loading(count) => {
                match count.current {
                    x if x == count.start => {
                        red_1.set_low();
                        red_2.set_low();
                        green.set_low();
                    },

                    x if x >= count.start / 4 * 3 => {
                        red_2.set_high()
                    },

                    x if x >= count.start / 4 * 2 => {
                        red_1.set_high()
                    }

                    x if x >= count.start / 4 => {
                        red_2.set_low()
                    },

                    x if x < count.start / 4 => {
                        red_1.set_low()
                    },

                    _ => (),
                }

                if count.current == 0 {
                    State::Idle
                } else {
                    State::Loading(count.decrement())
                }
            },

            State::Idle => {
                red_1.set_low();
                red_2.set_low();
                green.set_high();

                if button.is_low() {
                    State::Idle
                } else {
                    State::Phasers(Count::new(30))
                }
            },

            State::Phasers(count) => {
                match count.current {
                    x if x == count.start => {
                        red_1.set_low();
                        red_2.set_low();
                        green.set_low();
                    },

                    x if x % 3 == 0 => {
                        if red_1.is_set_low() {
                            red_1.set_high();
                            red_2.set_high();
                        } else {
                            red_1.set_low();
                            red_2.set_low();
                        }
                    },

                    _ => (),
                }


                if count.current == 0 {
                    State::Loading(Count::new(120))
                } else {
                    State::Phasers(count.decrement())
                }
            },
        };

        arduino_hal::delay_ms(16);
    }
}
