use std::f64::consts::{FRAC_PI_2, PI};

use crate::world::world_entity::ViewableEntity;

use super::util::normalize_range;
use super::util::TWO_PI;
use super::world_entity::WorldEntity;

#[derive(Copy, Clone)]
pub struct Camera {
    x_pos: f64,
    y_pos: f64,
    facing_direction: f64, // radians
    fov_angle: f64,
    fill_screen_distance: f64, // Distance between camera position and position where a wall should fill the screen
    horizon_distance: f64,
}

impl WorldEntity for Camera {
    fn x_pos(&self) -> f64 {
        self.x_pos
    }
    fn y_pos(&self) -> f64 {
        self.y_pos
    }
}

impl Camera {
    /// Constructs a new camera positioned at (0,0) with a facing angle of 0 and FOV of pi/4 (45 degrees).
    /// Distance to fill the screen is 1 and horizon distance is 100.
    pub fn new() -> Camera {
        Camera {
            x_pos: 0.0,
            y_pos: 0.0,
            facing_direction: 0.0,
            fov_angle: FRAC_PI_2,
            fill_screen_distance: 2.0,
            horizon_distance: 15.0,
        }
    }

    /// The angle at which the camera is facing
    pub fn facing_direction(&self) -> f64 {
        self.facing_direction
    }
    /// The angle of the camera's horizontal FOV
    pub fn fov_angle(&self) -> f64 {
        self.fov_angle
    }
    /// The distance from the camera an entity should be to fill the screen
    pub fn fill_screen_distance(&self) -> f64 {
        self.fill_screen_distance
    }
    /// The distance from the camera to the horizon line
    pub fn horizon_distance(&self) -> f64 {
        self.horizon_distance
    }

    /// Determines the angle from the center of the view frustum that the entity appears at to the camera
    pub fn view_angle_from_center(&self, other: &impl WorldEntity) -> f64 {
        let camera_vector_angle = (other.y_pos() - self.y_pos()).atan2(other.x_pos() - self.x_pos());

        return self.facing_direction - camera_vector_angle;
    }

    /// Returns true if the camera can see the other entity
    pub fn can_see(&self, other: &impl WorldEntity) -> bool {
        let angle_to_other = self.view_angle_from_center(other);
        let view_angle_from_center = normalize_range(angle_to_other, -PI..PI);
        let half_fov_angle = self.fov_angle / 2.0;

        return (-half_fov_angle..half_fov_angle).contains(&view_angle_from_center) && self.distance_to(other) < self.horizon_distance
    }

    /// Returns true if the camera can see the other entity using the entity's implementation
    pub fn can_see_viewable(&self, other: &impl ViewableEntity) -> bool {
        other.in_camera_view(self)
    }

    /// Returns an updated camera, moved forward diff_forward and rotated diff_angle
    pub fn update_cam(&self, diff_forward: f64, diff_angle: f64) -> Camera {
        let new_angle = normalize_range(self.facing_direction + diff_angle, 0.0..TWO_PI);

        let x_change = diff_forward * new_angle.cos();
        let y_change = diff_forward * new_angle.sin();

        let mut cam_copy = self.clone();
        cam_copy.x_pos = self.x_pos + x_change;
        cam_copy.y_pos = self.y_pos + y_change;
        cam_copy.facing_direction = new_angle;

        return cam_copy;
    }
}



