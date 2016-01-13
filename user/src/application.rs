#![feature(lang_items)]
#![no_std]

#[lang="stack_exhausted"]
extern fn stack_exhausted() {}

#[lang="eh_personality"]
extern fn eh_personality() {}

#[lang="panic_fmt"]
pub fn panic_fmt(_fmt: &core::fmt::Arguments, _file_line: &(&'static str, usize)) -> ! { loop { } }

#[allow(non_camel_case_types)]
pub enum PinMode {
    INPUT,
    OUTPUT,
    INPUT_PULLUP,
    INPUT_PULLDOWN,
    AF_OUTPUT_PUSHPULL, // Used internally for Alternate Function Output PushPull(TIM, UART, SPI etc)
    AF_OUTPUT_DRAIN,    // Used internally for Alternate Function Output Drain(I2C etc). External pullup resistors required.
    AN_INPUT,           // Used internally for ADC Input
    AN_OUTPUT,          // Used internally for DAC Output
    PIN_MODE_NONE = 0xFF
}

#[allow(non_camel_case_types)]
pub enum Pin {
    A0 = 10,
    A1 = 11,
    A2 = 12,
    D0 = 0,
    D1 = 1,
    D7 = 7
}

#[allow(non_camel_case_types)]
pub enum PinState {
    LOW = 0,
    HIGH = 1
}

use Pin::*;
use PinState::*;
use PinMode::*;

extern {
    fn HAL_Pin_Mode(pin: i16, mode: u8);
    fn HAL_GPIO_Write(pin: i16, value: u8);
    fn HAL_Delay_Milliseconds(delay: u32);
}

/// Set the pin mode
fn pin_mode(pin: Pin, mode: PinMode)
{
    unsafe {
        HAL_Pin_Mode(pin as i16, mode as u8);
    }
}

/// Write a value (HIGH or LOW) to the selected pin.
fn digital_write(pin: Pin, value: PinState)
{
    unsafe {
        HAL_GPIO_Write(pin as i16, value as u8);
   }
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
    pin_mode(D7, OUTPUT);
}

#[no_mangle]
#[export_name = "loop"]
pub fn main_loop()
{
    digital_write(D7, HIGH);
    delay(500);
    digital_write(D7, LOW);
    delay(500);
}
