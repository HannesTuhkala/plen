extern crate rand;

use ggez::nalgebra as na;
use serde_derive::{Serialize, Deserialize};
use ggez::graphics;
use ggez;
use crate::constants;
use crate::bullet;
use crate::assets::Assets;
use crate::math;

use crate::powerups::{PowerUpKind, AppliedPowerup};

#[derive(Serialize, Deserialize, Clone)]
pub enum PlaneType {
    SUKA_BLYAT_PLANE,
    HOWDY_COWBOY_PLANE,
    EL_POLLO_ROMERO_PLANE,
    ACHTUNG_BLITZ_PLANE,
}

impl PlaneType {
    pub fn speed(&self) -> f64 {
        match self {
            PlaneType::SUKA_BLYAT_PLANE => 5.4,
            PlaneType::HOWDY_COWBOY_PLANE => 1.2,
            PlaneType::EL_POLLO_ROMERO_PLANE => 1.1,
            PlaneType::ACHTUNG_BLITZ_PLANE => 1.3,
        }
    }

    pub fn agility(&self) -> f32 {
        match self {
            PlaneType::SUKA_BLYAT_PLANE => constants::DEFAULT_AGILITY * 23.,
            PlaneType::HOWDY_COWBOY_PLANE => constants::DEFAULT_AGILITY * 4.,
            PlaneType::EL_POLLO_ROMERO_PLANE => constants::DEFAULT_AGILITY * 2.,
            PlaneType::ACHTUNG_BLITZ_PLANE => constants::DEFAULT_AGILITY * 5.,
        }
    }

    pub fn firepower(&self) -> u8 {
        match self {
            PlaneType::SUKA_BLYAT_PLANE => constants::BULLET_DAMAGE * 5 as u8,
            PlaneType::HOWDY_COWBOY_PLANE => constants::BULLET_DAMAGE * 2. as u8,
            PlaneType::EL_POLLO_ROMERO_PLANE => constants::BULLET_DAMAGE * 2. as u8,
            PlaneType::ACHTUNG_BLITZ_PLANE => constants::BULLET_DAMAGE * 4. as u8,
        }
    }

    pub fn acceleration(&self) -> f32 {
        match self {
            PlaneType::SUKA_BLYAT_PLANE => constants::DEFAULT_ACCELERATION * 1.1,
            PlaneType::HOWDY_COWBOY_PLANE => constants::DEFAULT_ACCELERATION * 1.5,
            PlaneType::EL_POLLO_ROMERO_PLANE => constants::DEFAULT_ACCELERATION * 1.2,
            PlaneType::ACHTUNG_BLITZ_PLANE => constants::DEFAULT_ACCELERATION * 1.3,
        }
    }

    pub fn health(&self) -> u8 {
        match self {
            PlaneType::SUKA_BLYAT_PLANE => constants::DEFAULT_HEALTH * 1.1 as u8,
            PlaneType::HOWDY_COWBOY_PLANE => constants::DEFAULT_HEALTH * 1.3 as u8,
            PlaneType::EL_POLLO_ROMERO_PLANE => constants::DEFAULT_HEALTH * 1.2 as u8,
            PlaneType::ACHTUNG_BLITZ_PLANE => constants::DEFAULT_HEALTH * 1.2 as u8,
        }
    }

    pub fn resilience(&self) -> f32 {
        match self {
            PlaneType::SUKA_BLYAT_PLANE => 0.9,
            PlaneType::HOWDY_COWBOY_PLANE => 0.7,
            PlaneType::EL_POLLO_ROMERO_PLANE => 0.8,
            PlaneType::ACHTUNG_BLITZ_PLANE => 0.9,
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
    pub cooldown: f32,
    pub powerups: Vec<AppliedPowerup>,
    pub planetype: PlaneType,
}


impl Player {
    pub fn new(id: u64, position: na::Point2<f32>) -> Player {
        Player {
            id: id,
            rotation: 0.,
            angular_velocity: 0.,
            speed: 0.,
            health: PlaneType::SUKA_BLYAT_PLANE.health(),
            position: na::Point2::new(100.0, 100.0),
            velocity: na::Vector2::new(0.0, 0.0),
            powerups: vec!(),
            cooldown: 0.,
            planetype: PlaneType::SUKA_BLYAT_PLANE,
        }
    }

    pub fn update(&mut self, x_input: f32, y_input: f32, delta_time: f32) {
        self.cooldown = (self.cooldown - delta_time).max(0.);

        let mut dx = 0.;
        let mut dy = 0.;

        self.speed += y_input * constants::DEFAULT_ACCELERATION * delta_time;
        if self.speed > constants::MAX_SPEED {
            self.speed = constants::MAX_SPEED;
        }
        if self.speed < constants::MIN_SPEED {
            self.speed = constants::MIN_SPEED;
        }

        dx += self.speed * (self.rotation - std::f32::consts::PI/2.).cos();
        dy += self.speed * (self.rotation - std::f32::consts::PI/2.).sin();
        self.velocity = na::Vector2::new(dx, dy);

        self.position = math::wrap_around(
            self.position + self.velocity * delta_time
        );

        let angular_acceleration = x_input * constants::DEFAULT_AGILITY/10.;
        self.angular_velocity += angular_acceleration;
        self.angular_velocity *= constants::ANGULAR_FADE;
        if self.angular_velocity > constants::DEFAULT_AGILITY {
            self.angular_velocity = constants::DEFAULT_AGILITY;
        } else if self.angular_velocity < -constants::DEFAULT_AGILITY {
            self.angular_velocity = -constants::DEFAULT_AGILITY;
        }
        self.rotation = self.rotation + self.angular_velocity;

        self.manage_powerups(delta_time);
    }

    pub fn shoot(&mut self) -> Option<bullet::Bullet> {
        let dir = self.rotation - std::f32::consts::PI / 2.;

        if self.cooldown <= 0. {
            self.cooldown = constants::PLAYER_COOLDOWN;
            Some(bullet::Bullet::new(
                self.position,
                self.velocity + na::Vector2::new(
                    dir.cos() * constants::BULLET_VELOCITY,
                    dir.sin() * constants::BULLET_VELOCITY,
                ),
            ))
        } else {
            None
        }
    }

    pub fn apply_powerup(&mut self, kind: PowerUpKind) {
        // Only allow one weapon at a time
        if kind.is_weapon() {
            self.powerups.retain(|p| !p.kind.is_weapon())
        }
        else {
            self.powerups.retain(|p| p.kind != kind)
        }
        // Remove duplicates, only allow one weapon
        self.powerups.push(AppliedPowerup::new(kind))
    }

    pub fn manage_powerups(&mut self, delta: f32) {
        let new_powerups = self.powerups.iter_mut()
            .for_each(|p| {
                // Decrease the powerup time left
                p.duration_left = p.duration_left
                    .map(|left| left - delta)
            });

        // Check if the time is up and this should be removed
        self.powerups
            .retain(|p| {
                p.duration_left
                    .map(|left| left > 0.)
                    .unwrap_or(true)})
    }
}
