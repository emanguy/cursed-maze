use super::camera::Camera;
use super::world_entity::{ViewableEntity, WorldEntity};

pub struct Pillar {
    x_pos: f64,
    y_pos: f64,
}

/// Links two pillars to become a wall
pub struct Wall<'p1, 'p2> {
    pillar1: &'p1 Pillar,
    pillar2: &'p2 Pillar,
}

impl WorldEntity for Pillar {
    fn x_pos(&self) -> f64 {
        self.x_pos
    }
    fn y_pos(&self) -> f64 {
        self.y_pos
    }
}

impl Pillar {
    pub fn at(x_pos: f64, y_pos: f64) -> Pillar {
        Pillar { x_pos, y_pos }
    }
}

impl<'wall> ViewableEntity for Wall<'wall, 'wall> {
    fn in_camera_view(&self, camera: &Camera) -> bool {
        camera.can_see(self.pillar1) || camera.can_see(self.pillar2)
    }
}

impl<'p1, 'p2> Wall<'p1, 'p2> {
    pub fn from_pillars(pillar1: &'p1 Pillar, pillar2: &'p2 Pillar) -> Wall<'p1, 'p2> {
        Wall { pillar1, pillar2 }
    }

    pub fn pillar1(&self) -> &'p1 Pillar {
        self.pillar1
    }
    pub fn pillar2(&self) -> &'p2 Pillar {
        self.pillar2
    }
}
