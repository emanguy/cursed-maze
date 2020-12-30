use super::world_entity::WorldEntity;

pub struct Pillar {
    x_pos: f64,
    y_pos: f64,
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
