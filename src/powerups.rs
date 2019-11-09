use nalgebra as na;

use serde_derive::{Serialize, Deserialize};

use rand_derive::Rand;

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Rand)]
pub enum PowerUpKind {
    Missile,
    Afterburner,
    Laser
}
impl PowerUpKind {
    pub fn starting_duration(&self) -> Option<f32> {
        match self {
            PowerUpKind::Missile => None,
            PowerUpKind::Laser => None,
            PowerUpKind::Afterburner => Some(5.),
        }
    }

    pub fn is_weapon(&self) -> bool {
        match self {
            PowerUpKind::Missile => true,
            PowerUpKind::Laser => true,
            PowerUpKind::Afterburner => false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PowerUp {
    pub kind: PowerUpKind,
    pub position: na::Point2<f32>,
}

impl PowerUp {
    pub fn new(kind: PowerUpKind, position: na::Point2<f32>) -> Self {
        Self {
            kind,
            position,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AppliedPowerup {
    pub kind: PowerUpKind,
    pub duration_left: Option<f32>,
}

impl AppliedPowerup {
    pub fn new(kind: PowerUpKind) -> Self {
        let duration_left = kind.starting_duration();
        Self {
            kind,
            duration_left
        }
    }
}
