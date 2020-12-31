use super::camera::Camera;
use super::util::{normalize_range, TWO_PI};

pub trait WorldEntity {
    /// The x position of the entity
    fn x_pos(&self) -> f64;
    /// The y position of the entity
    fn y_pos(&self) -> f64;

    /// The distance from this entity to the other entity
    fn distance_to(&self, other: & dyn WorldEntity) -> f64 {
        let x_diff = other.x_pos() - self.x_pos();
        let y_diff = other.y_pos() - self.y_pos();

        (x_diff * x_diff - y_diff * y_diff).sqrt()
    }
}

pub trait ViewableEntity {
    fn in_camera_view(&self, camera: &Camera) -> bool;
}
