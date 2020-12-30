use std::thread::sleep;
use std::time::Duration;

use device_query::{DeviceQuery, DeviceState, Keycode};
use ncurses::*;

use curses_util::draw_2d::{Coordinate, draw_line};
use curses_util::lifecycle::CursesHandle;

use crate::world::camera::Camera;
use crate::world::pillar::Pillar;
use crate::world::world_entity::WorldEntity;

mod curses_util;
mod world;
mod input;

fn draw_pillar(screen_rows: i32, screen_cols: i32, camera: &Camera, pillar: &Pillar) {
    let pillar_dist = camera.distance_to(pillar);
    let pillar_ang = camera.view_angle_from_left(pillar);
    let half_screen_rows = screen_rows / 2;

    let horizon_rise = half_screen_rows as f64 * (1.0 - (pillar_dist - camera.fill_screen_distance()) / (camera.horizon_distance() - camera.fill_screen_distance()));
    let pillar_top = (half_screen_rows as f64 - horizon_rise) as i32;
    let pillar_bottom = (half_screen_rows as f64 + horizon_rise) as i32;
    let pillar_column = ((pillar_ang / camera.fov_angle()) * screen_cols as f64) as i32;

    let line_top = Coordinate { row: pillar_top, col: pillar_column };
    let line_bottom = Coordinate { row: pillar_bottom, col: pillar_column };

    draw_line(line_top, line_bottom);
}

fn main() {
    // When the curses handle falls out of scope it'll turn off curses
    let _curse_handle = CursesHandle::create();

    let mut max_row = 0;
    let mut max_col = 0;
    getmaxyx(stdscr(), &mut max_row, &mut max_col);

    let input = DeviceState::new();

    let cam = Camera::new();
    let pillar1 = Pillar::at(5.0, 2.5);
    let pillar2 = Pillar::at(25.0, 0.0);
    let pillar3 = Pillar::at(50.0, -25.0);
    let pillar4 = Pillar::at(55.0, 3.0);

    clear();
    draw_pillar(max_row, max_col, &cam, &pillar1);
    draw_pillar(max_row, max_col, &cam, &pillar2);
    draw_pillar(max_row, max_col, &cam, &pillar3);
    draw_pillar(max_row, max_col, &cam, &pillar4);
    refresh();

    loop {
        // Check for user quit
        let keyboard_state = input.get_keys();
        let mut quit = false;

        for key in keyboard_state {
            match key {
                Keycode::Enter => {
                    quit = true;
                },
                _ => {},
            };
        }
        // Consume input so it's not redirected to the terminal
        getch();
        // Wait till next frame
        sleep(Duration::from_millis(50));

        if quit {
            break;
        }
    }
}

