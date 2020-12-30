use device_query::DeviceState;
use ncurses::*;

use curses_util::lifecycle::CursesHandle;
use input::{move_camera, ProgramCommand};
use render::{frame_sleep, Scene};
use world::camera::Camera;
use world::pillar::Pillar;

mod curses_util;
mod world;
mod input;
mod render;


fn main() {
    // When the curses handle falls out of scope it'll turn off curses
    let _curse_handle = CursesHandle::create();

    let mut max_row = 0;
    let mut max_col = 0;
    getmaxyx(stdscr(), &mut max_row, &mut max_col);

    let input = DeviceState::new();

    let scene = Scene::with_dimensions(max_row, max_col);
    let mut cam = Camera::new();
    let pillar1 = Pillar::at(5.0, 2.5);
    let pillar2 = Pillar::at(25.0, 0.0);
    let pillar3 = Pillar::at(50.0, -25.0);
    let pillar4 = Pillar::at(55.0, 3.0);
    let pillars = vec!(pillar1, pillar2, pillar3, pillar4);

    loop {
        let (new_cam, command) = move_camera(&input, &cam);
        cam = new_cam;

        scene.render_frame(&cam, &pillars);

        // Wait till next frame
        frame_sleep();

        if command == ProgramCommand::Quit {
            break;
        }
    }
}

