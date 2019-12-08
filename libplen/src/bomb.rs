use nalgebra as na;
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
    pub position: na::Point2<f32>,
    pub status: BombStatus,
}

impl Bomb {
    pub fn new(position: na::Point2<f32>) -> Self {
        Self {
            status: BombStatus::Dropping(constants::BOMB_DROP_TIME),
            position,
        }
    }

    pub fn get_damage(position: na::Point2<f32>) -> f32 {

    }
}
