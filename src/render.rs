use std::f64::consts::PI;
use std::thread::sleep;
use std::time::Duration;

use ordered_float::NotNan;

use ncurses::*;

use super::curses_util::draw_2d::*;
use super::world::camera::Camera;
use super::world::pillar::{Pillar, Wall};
use super::world::util::{normalize_range};
use super::world::world_entity::WorldEntity;

pub const RENDER_FPS: f64 = 30.0;

pub fn frame_sleep() {
    sleep(Duration::from_millis((1000.0 / RENDER_FPS) as u64));
}

pub struct Scene {
    screen_rows: i32,
    screen_cols: i32,
}

#[derive(Copy, Clone)]
struct PillarCoords {
    line_top: Coordinate,
    line_bottom: Coordinate,
}

impl Scene {
    /// Creates a new scene with the given screen dimensions
    pub fn with_dimensions(screen_rows: i32, screen_cols: i32) -> Scene {
        Scene { screen_rows, screen_cols }
    }

    pub fn render_frame(&self, camera: &Camera, walls: &Vec<Wall>) {
        clear();

        let mut visible_walls: Vec<&Wall> = walls.iter().filter(|&wall| camera.can_see_viewable(wall)).collect();
        visible_walls.sort_by_cached_key(|&wall| {
            NotNan::new(camera.distance_to(wall)).expect("Distance to wall should not have been NaN but was")
        });
        visible_walls.reverse();

        for wall in visible_walls {
            if camera.can_see_viewable(wall) {
                let pillar1_screen_coords = self.calculate_pillar_coords(camera, wall.pillar1());
                let pillar2_screen_coords = self.calculate_pillar_coords(camera, wall.pillar2());

                let (left_pillar_coords, right_pillar_coords) = if pillar1_screen_coords.line_top.col <= pillar2_screen_coords.line_top.col {
                    (&pillar1_screen_coords, &pillar2_screen_coords)
                } else {
                    (&pillar2_screen_coords, &pillar1_screen_coords)
                };

                // Only fill if there is a space of at least one column between the pillars
                if right_pillar_coords.line_top.col - left_pillar_coords.line_top.col > 2 {
                    let top_left_fillshift = left_pillar_coords.line_top.coord_shift(1, 1);
                    let bottom_left_fillshift = left_pillar_coords.line_bottom.coord_shift(-1, 1);
                    let top_right_fillshift = right_pillar_coords.line_top.coord_shift(1, -1);
                    let bottom_right_fillshift = right_pillar_coords.line_bottom.coord_shift(-1, -1);

                    // TODO do something with the results here
                    let _ = fill_triangle(top_left_fillshift, bottom_left_fillshift, top_right_fillshift, '.');
                    let _ = fill_triangle(bottom_left_fillshift, top_right_fillshift, bottom_right_fillshift, '.');
                }

                draw_line(pillar1_screen_coords.line_top, pillar1_screen_coords.line_bottom, '#');
                draw_line(pillar2_screen_coords.line_top, pillar2_screen_coords.line_bottom, '#');
                draw_line(pillar1_screen_coords.line_top, pillar2_screen_coords.line_top, '#');
                draw_line(pillar1_screen_coords.line_bottom, pillar2_screen_coords.line_bottom, '#');
            }
        }

        refresh();
    }


    fn calculate_pillar_coords(&self, camera: &Camera, pillar: &Pillar) -> PillarCoords {
        let pillar_dist = camera.distance_to(pillar);
        let pillar_ang = normalize_range(camera.view_angle_from_center(pillar), -PI..PI);
        let half_screen_rows = self.screen_rows / 2;
        let half_screen_cols = self.screen_cols / 2;

        let horizon_rise = half_screen_rows as f64 * (1.0 - (pillar_dist - camera.fill_screen_distance()) / (camera.horizon_distance() - camera.fill_screen_distance()));
        let pillar_top = (half_screen_rows as f64 - horizon_rise) as i32;
        let pillar_bottom = (half_screen_rows as f64 + horizon_rise) as i32;
        let pillar_column = ((pillar_ang / camera.fov_angle()) * self.screen_cols as f64) as i32 + half_screen_cols;

        let line_top = Coordinate { row: pillar_top, col: pillar_column };
        let line_bottom = Coordinate { row: pillar_bottom, col: pillar_column };

        return PillarCoords { line_top, line_bottom };
    }
}

