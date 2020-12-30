use std::thread::sleep;
use std::time::Duration;

use ncurses::*;

use super::curses_util::draw_2d::*;
use super::world::camera::Camera;
use super::world::pillar::Pillar;
use super::world::util::{normalize_range, TWO_PI};
use super::world::world_entity::WorldEntity;

pub const RENDER_FPS: f64 = 30.0;

pub fn frame_sleep() {
    sleep(Duration::from_millis((1000.0 / RENDER_FPS) as u64));
}

pub struct Scene {
    screen_rows: i32,
    screen_cols: i32,
}

impl Scene {
    /// Creates a new scene with the given screen dimensions
    pub fn with_dimensions(screen_rows: i32, screen_cols: i32) -> Scene {
        Scene { screen_rows, screen_cols }
    }

    pub fn render_frame(&self, camera: &Camera, pillars: &Vec<Pillar>) {
        clear();

        for pillar in pillars {
            if camera.can_see(pillar) {
                self.draw_pillar(camera, pillar);
            }
        }

        refresh();
    }

    fn draw_pillar(&self, camera: &Camera, pillar: &Pillar) {
        let pillar_dist = camera.distance_to(pillar);
        let pillar_ang = normalize_range(camera.view_angle_from_left(pillar), 0.0..TWO_PI);
        let half_screen_rows = self.screen_rows / 2;

        let horizon_rise = half_screen_rows as f64 * (1.0 - (pillar_dist - camera.fill_screen_distance()) / (camera.horizon_distance() - camera.fill_screen_distance()));
        let pillar_top = (half_screen_rows as f64 - horizon_rise) as i32;
        let pillar_bottom = (half_screen_rows as f64 + horizon_rise) as i32;
        let pillar_column = ((pillar_ang / camera.fov_angle()) * self.screen_cols as f64) as i32;

        let line_top = Coordinate { row: pillar_top, col: pillar_column };
        let line_bottom = Coordinate { row: pillar_bottom, col: pillar_column };

        draw_line(line_top, line_bottom);
    }
}

