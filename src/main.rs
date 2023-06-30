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

fn increment_row(row: &mut usize, col: usize, direction: bool, display: &mut Display, timer: &mut Timer<TIMER0>) {
    let empty: [[u8; 5]; 5] = [
        [0, 0, 0, 0, 0,],
        [0, 0, 0, 0, 0,],
        [0, 0, 0, 0, 0,],
        [0, 0, 0, 0, 0,],
        [0, 0, 0, 0, 0,],
    ];
    if *row < 5 {
        for _ in 0..4 {
            let mut array = empty.clone();
            array[*row][col] = 1;
            display.show(timer, array, 100u32);
            // display.clear();
            drop(array);
            if direction {
                *row += 1;
            } else {
                *row -= 1;
            }
        }
    }
}

fn increment_col(row: usize, col: &mut usize, direction: bool, display: &mut Display, timer: &mut Timer<TIMER0>) {
    let empty: [[u8; 5]; 5] = [
        [0, 0, 0, 0, 0,],
        [0, 0, 0, 0, 0,],
        [0, 0, 0, 0, 0,],
        [0, 0, 0, 0, 0,],
        [0, 0, 0, 0, 0,],
    ];
    if *col < 5 {
        for _ in 0..4 {
            let mut array = empty.clone();
            array[row][*col] = 1;
            display.show(timer, array, 100u32);
            // display.clear();
            drop(array);
            if direction {
                *col += 1;
            } else {
                *col -= 1;
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
        increment_row(&mut row, col, true, &mut display, &mut timer);
        increment_col(row, &mut col, true, &mut display, &mut timer);
        increment_row(&mut row, col, false, &mut display, &mut timer);
        increment_col(row, &mut col, false, &mut display, &mut timer);
        i += 1;
    }
}