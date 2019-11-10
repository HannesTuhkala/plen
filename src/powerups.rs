use nalgebra as na;

use serde_derive::{Serialize, Deserialize};

use rand_derive::Rand;

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Rand)]
pub enum PowerUpKind {
    Afterburner,
    Laser,
    Health,
    Invincibility,
    Gun,
    SlowTime,
}
impl PowerUpKind {
    pub fn starting_duration(&self) -> Option<f32> {
        match self {
            PowerUpKind::Laser => None,
            PowerUpKind::Afterburner => Some(5.),
            PowerUpKind::Health => None,
            PowerUpKind::Invincibility => Some(10.),
            PowerUpKind::Gun => None,
            PowerUpKind::SlowTime => Some(10.),
        }
    }

    pub fn is_weapon(&self) -> bool {
        match self {
            PowerUpKind::Laser => true,
            PowerUpKind::Afterburner => false,
            PowerUpKind::Health => false,
            PowerUpKind::Invincibility => false,
            PowerUpKind::Gun => true,
            PowerUpKind::SlowTime => false,
        }
    }

    pub fn is_instant(&self) -> bool {
        match self {
            PowerUpKind::Laser => false,
            PowerUpKind::Afterburner => false,
            PowerUpKind::Health => true,
            PowerUpKind::Invincibility => false,
            PowerUpKind::Gun => false,
            PowerUpKind::SlowTime => false,
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
