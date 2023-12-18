use avr_device::interrupt;
use core::cell::RefCell;

type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;

pub const COMMUNICATION_SPEED: u32 = 57600; // Ravedude listen 57600 by default, instead of advised 9600 at the Starter Kit book

pub static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
    interrupt::Mutex::new(RefCell::new(None));

// macro_rules! print {
//     ($($t:tt)*) => {
//         interrupt::free(
//             |cs| {
//                 if let Some(console) = CONSOLE.borrow(cs).borrow_mut().as_mut() {
//                     let _ = ufmt::uwrite!(console, $($t)*);
//                 }
//             },
//         )
//     };
// }

macro_rules! println {
    ($($t:tt)*) => {
        avr_device::interrupt::free(
            |cs| {
                if let Some(console) = crate::print::CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    if let Err(_) = ufmt::uwriteln!(console, $($t)*) {
                      // Do nothing, could not debug write to usb connector
                    }
                }
            },
        )
    };
}

pub fn init(console: Console) {
    interrupt::free(|cs| {
        *CONSOLE.borrow(cs).borrow_mut() = Some(console);
    })
}
