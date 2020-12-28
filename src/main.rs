mod curses_util;

use ncurses::*;
use curses_util::draw_2d::{Coordinate, draw_line};
use curses_util::lifecycle::CursesHandle;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // When the curses handle falls out of scope it'll turn off curses
    let _curse_handle = CursesHandle::create();
    let hello_string = String::from("Hello, curses!");
    let half_strlen = (hello_string.len() as i32) / 2;

    let mut max_row = 0;
    let mut max_col = 0;
    getmaxyx(stdscr(), &mut max_row, &mut max_col);

    let original_start = Coordinate {row: 3, col: 3};
    let original_end = Coordinate {row: 10, col: 15};
    let mut row_change = 0;
    let mut row_direction = 1;

    let longboi_start = Coordinate {row: max_row - 20, col: 3};
    let longboi_end = Coordinate {row: max_row - 15, col: 3};
    let longboi_change_limit = max_col - longboi_start.col - 5;
    let mut longboi_change = 0;
    let mut longboi_direction = 1;

    loop {
        let current_start = original_start.coord_shift(row_change, 0);
        let current_end = original_end.coord_shift(-row_change, 0);
        let current_long_end = longboi_end.coord_shift(0, longboi_change);

        // Do drawing
        clear();
        mvprintw(max_row / 2, (max_col / 2) - half_strlen, &hello_string);
        draw_line(current_start, current_end);
        draw_line(longboi_start, current_long_end);
        refresh();

        // Update end positions of line
        row_change = row_change + row_direction;
        longboi_change = longboi_change + longboi_direction;

        if row_change == 0 || row_change == 7 {
            row_direction = -row_direction;
        }
        if longboi_change == 0 || longboi_change == longboi_change_limit {
            longboi_direction = -longboi_direction;
        }

        // Wait till next frame
        sleep(Duration::from_millis(50));

        // Check for user quit
        if getch() == '\n' as i32 {
            break
        }
    }
}

