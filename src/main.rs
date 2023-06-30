#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use microbit::{
    board::Board,
    pac::TIMER0,
    hal::{prelude::*, timer::Timer},
    display::blocking::Display,
};
use cortex_m_semihosting::hprintln;

fn increment_led(
    row: &mut usize,
    col: &mut usize,
    use_row: bool,
    increase: bool,
    display: &mut Display,
    timer: &mut Timer<TIMER0>)
{
    let empty: [[u8; 5]; 5] = [
        [0, 0, 0, 0, 0,],
        [0, 0, 0, 0, 0,],
        [0, 0, 0, 0, 0,],
        [0, 0, 0, 0, 0,],
        [0, 0, 0, 0, 0,],
    ];
    if *row < 5 && *col < 5 {
        for _ in 0..4 {
            let mut array = empty.clone();
            array[*row][*col] = 1;
            display.show(timer, array, 30u32);
            // display.clear();
            drop(array);
            if increase {
                if use_row {
                    *row += 1;
                } else {
                    *col += 1;
                }
            } else {
                if use_row {
                    *row -= 1;
                } else {
                    *col -= 1;
                }
            }
        }
    }
}

#[entry]
fn main() -> ! {
    // Variable setup
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    // Main loop with counter
    let mut i = 0;
    let mut col = 0;
    let mut row = 0;
    loop {
        // hprintln!("cycle {}", i);
        let increment_upwards = true;
        let increment_downwards = false;
        let update_row = true;
        let update_col = false;
        increment_led(&mut row, &mut col, update_row, increment_upwards, &mut display, &mut timer);
        increment_led(&mut row, &mut col, update_col, increment_upwards, &mut display, &mut timer);
        increment_led(&mut row, &mut col, update_row, increment_downwards, &mut display, &mut timer);
        increment_led(&mut row, &mut col, update_col, increment_downwards, &mut display, &mut timer);
        i += 1;
    }
}