extern crate rand;

use ggez::nalgebra as na;
use serde_derive::{Serialize, Deserialize};
use ggez::graphics;
use ggez;
use crate::constants;
use crate::bullet;
use crate::assets::Assets;

use crate::powerups::PowerUpKind;

#[derive(Serialize, Deserialize, Clone)]
enum PlaneType {
    SUKA_BLYAT_PLANE,
    HOWDY_COWBOY_PLANE,
    EL_POLLO_ROMERO_PLANE,
    ACHTUNG_BLITZ_PLANE,
}

impl PlaneType {
    pub fn speed() -> f64 {
        match PlaneType {
            PlaneType::SUKA_BLYAT_PLANE => 20.,
            PlaneType::HOWDY_COWBOY_PLANE => 10.,
            PlaneType::EL_POLLO_ROMERO_PLANE => 5.,
            PlaneType::ACHTUNG_BLITZ_PLANE => 15.,
        }
    }

    pub fn agility() -> f64 {
        match PlaneType {
            PlaneType::SUKA_BLYAT_PLANE => 3.,
            PlaneType::HOWDY_COWBOY_PLANE => 4.,
            PlaneType::EL_POLLO_ROMERO_PLANE => 2.,
            PlaneType::ACHTUNG_BLITZ_PLANE => 5.,
        }
    }

    pub fn firepower() -> f64 {
        match PlaneType {
            PlaneType::SUKA_BLYAT_PLANE => 300.,
            PlaneType::HOWDY_COWBOY_PLANE => 200.,
            PlaneType::EL_POLLO_ROMERO_PLANE => 200.,
            PlaneType::ACHTUNG_BLITZ_PLANE => 270.,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: u64,
    pub rotation: f32,
    pub angular_velocity: f32,
    pub speed: f32,
    pub health: u8,
    pub position: na::Point2<f32>,
    pub velocity: na::Vector2<f32>,
    pub powerups: Vec<PowerUpKind>,
    pub planetype: PlaneType,
}


impl Player {
    pub fn new(id: u64, position: na::Point2<f32>) -> Player {
        Player {
            id: id,
            rotation: 0.,
            angular_velocity: 0.,
            speed: 0.,
            health: 100,
            position: na::Point2::new(100.0, 100.0),
            velocity: na::Vector2::new(0.0, 0.0),
            powerups: vec!(),
            planetype: PlaneType::SUKA_BLYAT_PLANE,
        }
    }

    pub fn shoot(&self) -> bullet::Bullet {
        let dir = self.rotation - std::f32::consts::PI / 2.;

        bullet::Bullet::new(
            self.position,
            self.velocity + na::Vector2::new(
                dir.cos() * constants::BULLET_VELOCITY,
                dir.sin() * constants::BULLET_VELOCITY,
            ),
        )
    }

    pub fn apply_powerup(&mut self, kind: PowerUpKind) {
        self.powerups.push(kind)
    }
}
