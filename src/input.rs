use std::f64::consts::FRAC_PI_2;

use device_query::{DeviceQuery, DeviceState, Keycode};
use ncurses::getch;

use super::render::RENDER_FPS;
use super::world::camera::Camera;

#[derive(Eq, PartialEq)]
pub enum ProgramCommand {
    NoCommand,
    Quit,
}

/// Based on the state of the input device, move the camera accordingly.
///
/// Returns the updated camera and a boolean saying whether or not the program should be quit.
pub fn move_camera(input: &DeviceState, camera_entity: &Camera) -> (Camera, ProgramCommand) {
    let keys_pressed = input.get_keys();
    let mut command = ProgramCommand::NoCommand;
    let mut forward_change = 0.0;
    let mut angle_change = 0.0;

    // Consume input so it's not redirected to the terminal
    getch();

    for key in keys_pressed {
        match key {
            Keycode::W | Keycode::Up => forward_change = forward_change + 4.0 / RENDER_FPS,
            Keycode::S | Keycode::Down => forward_change = forward_change - 4.0 / RENDER_FPS,
            Keycode::A | Keycode::Left => angle_change = angle_change + FRAC_PI_2 / RENDER_FPS,
            Keycode::D | Keycode::Right => angle_change = angle_change - FRAC_PI_2 / RENDER_FPS,
            Keycode::Escape | Keycode::Q => command = ProgramCommand::Quit,
            _ => {},
        }
    }

    return (camera_entity.update_cam(forward_change, angle_change), command);
}
