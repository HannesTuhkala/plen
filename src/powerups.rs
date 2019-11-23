use nalgebra as na;

use serde_derive::{Serialize, Deserialize};

use rand_derive::Rand;

extern crate strum;
extern crate strum_macros;
use strum_macros::{Display, EnumIter};
use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, EnumIter, Clone, Hash, PartialEq, Eq, Rand, Debug)]
pub enum PowerUpKind {
    Afterburner,
    Laser,
    Health,
    Invincibility,
    Gun,
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
            PowerUpKind::SlowTime => false,
            PowerUpKind::Invisible => false,
        }
    }

    pub fn get_likelyhood(&self) -> i32 {
        match self {
            PowerUpKind::Laser => 60,
            PowerUpKind::Afterburner => 40,
            PowerUpKind::Health => 40,
            PowerUpKind::Invincibility => 30,
            PowerUpKind::Gun => 70,
            PowerUpKind::SlowTime => 10,
            PowerUpKind::Invisible => 60,
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
