use device_query::DeviceState;
use ncurses::*;

use curses_util::lifecycle::CursesHandle;
use input::{move_camera, ProgramCommand};
use render::{frame_sleep, Scene};
use world::camera::Camera;
use world::pillar::{Pillar, Wall};
use crate::maze::generation::Maze;
use crate::maze::world_translation::{create_pillars_for_maze, create_walls_for_maze};

mod curses_util;
mod world;
mod input;
mod render;
mod maze;


fn main() {
    let maze_creation_result = Maze::new(10, 10, 5);
    let generated_maze = match maze_creation_result {
        Ok(maze) => maze,
        Err(generate_err) => {
            println!("Maze generation failed: {}", generate_err);
            return;
        },
    };

    // When the curses handle falls out of scope it'll turn off curses
    let _curse_handle = CursesHandle::create();

    let mut max_row = 0;
    let mut max_col = 0;
    getmaxyx(stdscr(), &mut max_row, &mut max_col);

    let input = DeviceState::new();

    let scene = Scene::with_dimensions(max_row, max_col);
    let mut cam = Camera::new();

    let pillars = create_pillars_for_maze(&generated_maze);
    let walls = create_walls_for_maze(&generated_maze, &pillars);

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

