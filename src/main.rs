use std::thread::sleep;
use std::time::Duration;

use device_query::{DeviceQuery, DeviceState, Keycode};
use ncurses::*;

use curses_util::draw_2d::{Coordinate, draw_line};
use curses_util::lifecycle::CursesHandle;

mod curses_util;

fn main() {
    // When the curses handle falls out of scope it'll turn off curses
    let _curse_handle = CursesHandle::create();
    let hello_string = String::from("Hello, curses!");
    let half_strlen = (hello_string.len() as i32) / 2;

    let mut max_row = 0;
    let mut max_col = 0;
    getmaxyx(stdscr(), &mut max_row, &mut max_col);

    let input = DeviceState::new();

    let original_start = Coordinate {row: 3, col: 3};
    let original_end = Coordinate {row: 10, col: 15};
    let mut row_change = 0;
    let mut row_direction = 1;

    let longboi_start = Coordinate {row: max_row - 20, col: max_col / 2};
    let longboi_end = Coordinate {row: max_row - 15, col: 5};
    let longboi_change_limit = max_col - 10;
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

        // Update end positions of line
        row_change = row_change + row_direction;
        longboi_change = longboi_change + longboi_direction;

        if row_change == 0 || row_change == 7 {
            row_direction = -row_direction;
        }
        if longboi_change == 0 || longboi_change == longboi_change_limit {
            longboi_direction = -longboi_direction;
        }


        // Check for user quit
        let keyboard_state = input.get_keys();
        let mut quit = false;
        let mut pressed_keys = String::from("Commands: ");

        if !pressed_keys.is_empty() {
            for key in keyboard_state {
                match key {
                    Keycode::Up | Keycode::W => pressed_keys = pressed_keys + "FORWARD,",
                    Keycode::Down | Keycode::S => pressed_keys = pressed_keys + "BACKWARD,",
                    Keycode::Left | Keycode::A => pressed_keys = pressed_keys + "LEFT,",
                    Keycode::Right | Keycode::D => pressed_keys = pressed_keys + "RIGHT,",
                    Keycode::Enter => {
                        pressed_keys = pressed_keys + "QUIT,";
                        quit = true;
                    },
                    _ => pressed_keys = pressed_keys + format!("OTHER({}),", key).as_str(),
                };
            }
        }
        // Consume input so it's not redirected to the terminal
        getch();

        mvprintw(max_row-1, 0, pressed_keys.as_str());
        refresh();

        // Wait till next frame
        sleep(Duration::from_millis(50));

        if quit {
            break;
        }
    }
}

