#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

mod millis;

use arduino_hal::prelude::*;
use millis::*;
use panic_halt as _;
use ufmt::{uwrite, uwriteln};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let button = pins.d2;
    let mut led = pins.d13.into_output();

    millis_init(dp.TC0);

    unsafe { avr_device::interrupt::enable() }

    let mut held = false;
    let mut held_start = 0;
    let mut space_start = 0;
    let mut waiting = false;

    let mut signal_counter = 0;
    let mut signal_buffer: [u32; 6] = [0, 0, 0, 0, 0, 0];

    loop {
        if signal_counter > 5 {
            uwriteln!(&mut serial, "Too many presses").void_unwrap();
            signal_counter = 0;
            signal_buffer = [0; 6];
        }

        if button.is_high() {
            if !held {
                waiting = false;
                held = true;
                held_start = millis();
            }

            led.set_high();
        }

        if button.is_low() {
            if held {
                held = false;

                if millis() - held_start < 250 {
                    signal_buffer[signal_counter] = 1;
                } else {
                    signal_buffer[signal_counter] = 2;
                }

                signal_counter += 1;

                space_start = millis();
            }

            led.set_low();
        }

        if !held {
            if waiting {
                continue;
            }

            if millis() - space_start >= 1000 && millis() - space_start < 1500 {
                if signal_counter != 0 {
                    for i in signal_counter..5 {
                        signal_buffer[i] = 0
                    }

                    uwrite!(&mut serial, "{}", match_character(&signal_buffer)).void_unwrap();
                    signal_counter = 0;
                    signal_buffer = [0; 6];
                }
            }
            if millis() - space_start >= 2500 {
                uwrite!(&mut serial, " ").void_unwrap();

                waiting = true
            }
        }
    }
}

fn match_character(signals: &[u32; 6]) -> char {
    match signals {
        [1, 2, 0, 0, 0, 0] => 'A',
        [2, 1, 1, 1, 0, 0] => 'B',
        [2, 1, 2, 1, 0, 0] => 'C',
        [2, 1, 1, 0, 0, 0] => 'D',
        [1, 0, 0, 0, 0, 0] => 'E',
        [1, 1, 2, 1, 0, 0] => 'F',
        [2, 2, 1, 0, 0, 0] => 'G',
        [1, 1, 1, 1, 0, 0] => 'H',
        [1, 1, 0, 0, 0, 0] => 'I',
        [1, 2, 2, 2, 0, 0] => 'J',
        [2, 1, 2, 0, 0, 0] => 'K',
        [1, 2, 1, 1, 0, 0] => 'L',
        [2, 2, 0, 0, 0, 0] => 'M',
        [2, 1, 0, 0, 0, 0] => 'N',
        [2, 2, 2, 0, 0, 0] => 'O',
        [1, 2, 2, 1, 0, 0] => 'P',
        [2, 2, 1, 2, 0, 0] => 'Q',
        [1, 2, 1, 0, 0, 0] => 'R',
        [1, 1, 1, 0, 0, 0] => 'S',
        [2, 0, 0, 0, 0, 0] => 'T',
        [1, 1, 2, 0, 0, 0] => 'U',
        [1, 1, 1, 2, 0, 0] => 'V',
        [1, 2, 2, 0, 0, 0] => 'W',
        [2, 1, 1, 2, 0, 0] => 'X',
        [2, 1, 2, 2, 0, 0] => 'Y',
        [2, 2, 1, 1, 0, 0] => 'Z',

        [2, 2, 2, 2, 2, 0] => '0',
        [1, 2, 2, 2, 2, 0] => '1',
        [1, 1, 2, 2, 2, 0] => '2',
        [1, 1, 1, 2, 2, 0] => '3',
        [1, 1, 1, 1, 2, 0] => '4',
        [1, 1, 1, 1, 1, 0] => '5',
        [2, 1, 1, 1, 1, 0] => '6',
        [2, 2, 1, 1, 1, 0] => '7',
        [2, 2, 2, 1, 1, 0] => '8',
        [2, 2, 2, 2, 1, 0] => '9',

        _ => '#',
    }
}
