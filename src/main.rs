#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use microbit::{
    board::Board,
    hal::prelude::*,
    hal::timer::Timer,
};
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    // Variable setup
    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);

    // Main loop with counter
    let mut i = 0;
    board.display_pins.col1.set_low().unwrap();
    loop {
        hprintln!("cycle {}", i);
        board.display_pins.row1.set_high().unwrap();
        timer.delay_ms(1000u16);
        board.display_pins.row1.set_low().unwrap();
        timer.delay_ms(1000u16);
        i += 1;
    }
}