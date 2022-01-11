use device_query::DeviceState;
use ncurses::*;

use curses_util::lifecycle::CursesHandle;
use input::{move_camera, ProgramCommand};
use render::{frame_sleep, Scene};
use world::camera::Camera;
use world::pillar::{Pillar, Wall};

mod curses_util;
mod world;
mod input;
mod render;
mod maze;


fn main() {
    // When the curses handle falls out of scope it'll turn off curses
    let _curse_handle = CursesHandle::create();

    let mut max_row = 0;
    let mut max_col = 0;
    getmaxyx(stdscr(), &mut max_row, &mut max_col);

    let input = DeviceState::new();

    let scene = Scene::with_dimensions(max_row, max_col);
    let mut cam = Camera::new();
    let mut pillar_set_1: Vec<Pillar> = Vec::new();
    let mut pillar_set_2: Vec<Pillar> = Vec::new();

    // Pillar set 1 horizontal segment
    for x_coord in (0..=8).step_by(2) {
        pillar_set_1.push(Pillar::at(x_coord as f64, 2.0));
    }

    // Pillar set 1 vertical segment
    for y_coord in (4..=10).step_by(2) {
        pillar_set_1.push(Pillar::at(8.0, y_coord as f64))
    }

    // Pillar set 2 horizontal segment
    for x_coord in (0..=12).step_by(2) {
        pillar_set_2.push(Pillar::at(x_coord as f64, -2.0));
    }

    // Pillar set 2 vertical segment
    for y_coord in (0..=10).step_by(2) {
        pillar_set_2.push(Pillar::at(12.0, y_coord as f64));
    }

    // Create all walls from pillars
    let mut walls: Vec<Wall> = Vec::new();

    for pillar_idx in 0..(pillar_set_1.len() - 1) {
        walls.push(Wall::from_pillars(pillar_set_1.get(pillar_idx).unwrap(), pillar_set_1.get(pillar_idx + 1).unwrap()));
    }
    for pillar_idx in 0..(pillar_set_2.len() - 1) {
        walls.push(Wall::from_pillars(pillar_set_2.get(pillar_idx).unwrap(), pillar_set_2.get(pillar_idx + 1).unwrap()));
    }

    walls.reverse();

    loop {
        let (new_cam, command) = move_camera(&input, &cam);
        cam = new_cam;

        scene.render_frame(&cam, &walls);

        // Wait till next frame
        frame_sleep();

        if command == ProgramCommand::Quit {
            break;
        }
    }
}

