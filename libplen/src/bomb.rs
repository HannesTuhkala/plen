use crate::math;
use crate::constants;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum BombStatus {
    Dropping(f32),
    Detonating,
    Exploding(f32),
    Dead,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Bomb {
    pub position: math::Vec2,
    pub velocity: math::Vec2,
    pub status: BombStatus,
}

impl Bomb {
    pub fn new(velocity: math::Vec2, position: math::Vec2) -> Self {
        Self {
            position,
            velocity,
            status: BombStatus::Dropping(constants::BOMB_DROP_TIME),
        }
    }

    pub fn get_damage(&self, position: math::Vec2) -> f32 {
        let center_to_point = math::find_closest_vector_to_point(
            self.position, position
        );
        let dist = (center_to_point.x.powi(2) + center_to_point.y.powi(2)).sqrt();
        if dist > constants::BOMB_BLAST_RADIUS {
            0.
        } else {
            constants::BOMB_MAX_DAMAGE*(constants::BOMB_BLAST_RADIUS - dist)
                / constants::BOMB_BLAST_RADIUS
        }
    }
}
