use enum_map::Enum;
use rand_derive::Rand;
use serde_derive::{Serialize, Deserialize};
use strum_macros::EnumIter;

use crate::math::Vec2;

#[derive(Serialize, Deserialize, EnumIter, Copy, Clone, Hash, PartialEq, Eq, Rand, Debug, Enum)]
pub enum PowerUpKind {
    Afterburner,
    Laser,
    Health,
    Invincibility,
    Gun,
    Missile,
    SlowTime,
    Invisible,
}

impl PowerUpKind {
    pub fn starting_duration(&self) -> Option<f32> {
        match self {
            PowerUpKind::Laser => None,
            PowerUpKind::Afterburner => Some(5.),
            PowerUpKind::Health => None,
            PowerUpKind::Invincibility => Some(7.),
            PowerUpKind::Gun => None,
            PowerUpKind::Missile => None,
            PowerUpKind::SlowTime => Some(1.5),
            PowerUpKind::Invisible => Some(7.),
        }
    }

    pub fn is_weapon(&self) -> bool {
        match self {
            PowerUpKind::Laser => true,
            PowerUpKind::Afterburner => false,
            PowerUpKind::Health => false,
            PowerUpKind::Invincibility => false,
            PowerUpKind::Gun => true,
            PowerUpKind::Missile => true,
            PowerUpKind::SlowTime => false,
            PowerUpKind::Invisible => false,
        }
    }

    pub fn is_instant(&self) -> bool {
        match self {
            PowerUpKind::Laser => false,
            PowerUpKind::Afterburner => false,
            PowerUpKind::Health => true,
            PowerUpKind::Invincibility => false,
            PowerUpKind::Gun => false,
            PowerUpKind::Missile => false,
            PowerUpKind::SlowTime => false,
            PowerUpKind::Invisible => false,
        }
    }

    pub fn is_triggerable(&self) -> bool {
        match self {
            PowerUpKind::Laser => false,
            PowerUpKind::Afterburner => true,
            PowerUpKind::Health => false,
            PowerUpKind::Invincibility => true,
            PowerUpKind::Gun => false,
            PowerUpKind::Missile => false,
            PowerUpKind::SlowTime => true,
            PowerUpKind::Invisible => true,
        }
    }

    pub fn get_likelihood(&self) -> i32 {
        match self {
            PowerUpKind::Laser => 60,
            PowerUpKind::Afterburner => 70,
            PowerUpKind::Health => 40,
            PowerUpKind::Invincibility => 30,
            PowerUpKind::Gun => 70,
            PowerUpKind::Missile => 60,
            PowerUpKind::SlowTime => 10,
            PowerUpKind::Invisible => 60,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PowerUp {
    pub kind: PowerUpKind,
    pub position: Vec2,
}

impl PowerUp {
    pub fn new(kind: PowerUpKind, position: Vec2) -> Self {
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
