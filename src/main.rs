use device_query::DeviceState;
use ncurses::*;

use curses_util::draw_2d::{Coordinate, fill_triangle};
use curses_util::lifecycle::CursesHandle;
use input::{move_camera, ProgramCommand};
use render::{frame_sleep, Scene};
use world::camera::Camera;
use world::pillar::Pillar;

use crate::curses_util::draw_2d::draw_line;

mod curses_util;
mod world;
mod input;
mod render;


fn main() {
    // When the curses handle falls out of scope it'll turn off curses
    let _curse_handle = CursesHandle::create();

    let point_up_triangle = (Coordinate { row: 10, col: 5 }, Coordinate { row: 5, col: 10 }, Coordinate { row: 7, col: 15 });
    let point_down_triangle = (Coordinate { row: 5, col: 20 }, Coordinate { row: 10, col: 25 }, Coordinate { row: 7, col: 30 });
    let flat_left_triangle = (Coordinate { row: 15, col: 5 }, Coordinate { row: 25, col: 5 }, Coordinate { row: 20, col: 15 });
    let flat_right_triangle = (Coordinate { row: 20, col: 20 }, Coordinate { row: 15, col: 30 }, Coordinate { row: 25, col: 30 });
    let wall_coords = (Coordinate { row: 5, col: 50 }, Coordinate { row: 55, col: 50 }, Coordinate { row: 15, col: 70 }, Coordinate { row: 45, col: 70 });
    let tuple_fill = |coord_tuple: (Coordinate, Coordinate, Coordinate), fill_char: char| fill_triangle(coord_tuple.0, coord_tuple.1, coord_tuple.2, fill_char);

    clear();
    tuple_fill(point_up_triangle, '@');
    tuple_fill(point_down_triangle, '@');
    tuple_fill(flat_left_triangle, '@');
    tuple_fill(flat_right_triangle, '@');

    fill_triangle(wall_coords.0.coord_shift(1, 1), wall_coords.1.coord_shift(-1, 1), wall_coords.2.coord_shift(1, -1), '-');
    fill_triangle(wall_coords.1.coord_shift(-1, 1), wall_coords.2.coord_shift(1, -1), wall_coords.3.coord_shift(-1, -1), '_');
    draw_line(wall_coords.0, wall_coords.1, '#');
    draw_line(wall_coords.0, wall_coords.2, '#');
    draw_line(wall_coords.1, wall_coords.3, '#');
    draw_line(wall_coords.2, wall_coords.3, '#');
    refresh();

    loop {
        // Wait till next frame
        frame_sleep();

        if getch() == '\n' as i32 {
            break;
        }
    }
}

