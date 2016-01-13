#![feature(lang_items)]
#![no_std]

#[lang="stack_exhausted"]
extern fn stack_exhausted() {}

#[lang="eh_personality"]
extern fn eh_personality() {}

#[lang="panic_fmt"]
pub fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> ! { loop { } }


pub mod gpio;

use gpio::{Pin, PinState, PinMode};


extern {
    fn HAL_Delay_Milliseconds(delay: u32);
}


/// Sleep for the specified number of milliseconds.
fn delay(delay: u32) {
    unsafe {
        HAL_Delay_Milliseconds(delay);
    }
}

#[no_mangle]
pub fn setup()
{
    gpio::pin_mode(Pin::D7, PinMode::OUTPUT);
}

#[no_mangle]
#[export_name = "loop"]
pub fn main_loop()
{
    for _ in 0..2 {
        gpio::digital_write(Pin::D7, PinState::HIGH);
        delay(500);
        gpio::digital_write(Pin::D7, PinState::LOW);
        delay(500);
    }
    for _ in 0..2 {
        gpio::digital_write(Pin::D7, PinState::HIGH);
        delay(200);
        gpio::digital_write(Pin::D7, PinState::LOW);
        delay(200);
    }
}
