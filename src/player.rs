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

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum PlaneType {
    SukaBlyat,
    HowdyCowboy,
    ElPolloRomero,
    AchtungBlitz,
}

impl PlaneType {
    pub fn speed(&self) -> f64 {
        match self {
            PlaneType::SukaBlyat => 5.4,
            PlaneType::HowdyCowboy => 1.2,
            PlaneType::ElPolloRomero => 1.1,
            PlaneType::AchtungBlitz => 1.3,
        }
    }

    pub fn agility(&self) -> f32 {
        match self {
            PlaneType::SukaBlyat => constants::DEFAULT_AGILITY * 5.,
            PlaneType::HowdyCowboy => constants::DEFAULT_AGILITY * 4.,
            PlaneType::ElPolloRomero => constants::DEFAULT_AGILITY * 2.,
            PlaneType::AchtungBlitz => constants::DEFAULT_AGILITY * 5.,
        }
    }

    pub fn firepower(&self) -> i16 {
        match self {
            PlaneType::SukaBlyat => constants::BULLET_DAMAGE * 5 as i16,
            PlaneType::HowdyCowboy => constants::BULLET_DAMAGE * 2. as i16,
            PlaneType::ElPolloRomero => constants::BULLET_DAMAGE * 2. as i16,
            PlaneType::AchtungBlitz => constants::BULLET_DAMAGE * 4. as i16,
        }
    }

    pub fn acceleration(&self) -> f32 {
        match self {
            PlaneType::SukaBlyat => constants::DEFAULT_ACCELERATION * 1.1,
            PlaneType::HowdyCowboy => constants::DEFAULT_ACCELERATION * 1.5,
            PlaneType::ElPolloRomero => constants::DEFAULT_ACCELERATION * 1.2,
            PlaneType::AchtungBlitz => constants::DEFAULT_ACCELERATION * 1.3,
        }
    }

    pub fn health(&self) -> i16 {
        match self {
            PlaneType::SukaBlyat => constants::DEFAULT_HEALTH * 1.1 as i16,
            PlaneType::HowdyCowboy => constants::DEFAULT_HEALTH * 1.3 as i16,
            PlaneType::ElPolloRomero => constants::DEFAULT_HEALTH * 1.2 as i16,
            PlaneType::AchtungBlitz => constants::DEFAULT_HEALTH * 1.2 as i16,
        }
    }

    pub fn resilience(&self) -> f32 {
        match self {
            PlaneType::SukaBlyat => 0.9,
            PlaneType::HowdyCowboy => 0.7,
            PlaneType::ElPolloRomero => 0.8,
            PlaneType::AchtungBlitz => 0.9,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            PlaneType::SukaBlyat => "Suka Blyat",
            PlaneType::HowdyCowboy => "Howdy Cowboy",
            PlaneType::ElPolloRomero => "El Pllo Romero",
            PlaneType::AchtungBlitz => "Achtung Blitz",
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
}

impl Color {
    pub fn rgba(&self) -> (f32, f32, f32, f32) {
        match self {
            Color::Red => (1., 0., 0., 1.),
            Color::Green => (0., 1., 0., 1.),
            Color::Blue => (0., 0., 1., 1.),
            Color::Yellow => (1., 1., 0., 1.),
            Color::Purple => (1., 0., 1., 1.),
        }
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub id: u64,
    pub rotation: f32,
    pub angular_velocity: f32,
    pub speed: f32,
    pub health: i16,
    pub position: na::Point2<f32>,
    pub cooldown: f32,
    pub powerups: Vec<AppliedPowerup>,
    pub planetype: PlaneType,
    pub color: Color,
    pub name: String
}


impl Player {
    pub fn new(id: u64, position: na::Point2<f32>,
               plane_type: PlaneType, color: Color,
               name: String) -> Player {
        Player {
            id: id,
            rotation: 0.,
            angular_velocity: 0.,
            speed: 0.,
            health: plane_type.health(),
            position: na::Point2::new(100.0, 100.0),
            powerups: vec!(),
            cooldown: 0.,
            planetype: plane_type,
            color: color,
            name: name
        }
    }

    pub fn update(&mut self, x_input: f32, y_input: f32, delta_time: f32) {
        self.cooldown = (self.cooldown - delta_time).max(0.);

        let velocity = self.final_velocity();

        self.speed += y_input * self.planetype.acceleration() * delta_time;

        self.position = math::wrap_around(
            self.position + velocity * delta_time
        );

        let angular_acceleration = x_input * self.planetype.agility()/10.;
        self.angular_velocity += angular_acceleration;
        self.angular_velocity *= constants::ANGULAR_FADE;
        if self.angular_velocity > self.planetype.agility() {
            self.angular_velocity = self.planetype.agility();
        } else if self.angular_velocity < -self.planetype.agility() {
            self.angular_velocity = -self.planetype.agility();
        }
        self.rotation = (self.rotation + self.angular_velocity * delta_time);

        self.manage_powerups(delta_time);
    }

    pub fn damage_player(&mut self, damage: i16) {
        self.health -= damage;

        if self.health <= 0 {
            self.health = 0;
        }
        if self.health > self.max_health() {
            self.health = self.max_health()
        }
    }

    pub fn shoot(&mut self) -> Option<bullet::Bullet> {
        let dir = self.rotation - std::f32::consts::PI / 2.;

        if self.cooldown <= 0. {
            self.cooldown = constants::PLAYER_COOLDOWN;
            Some(bullet::Bullet::new(
                self.position + na::Vector2::new(
                    dir.cos() * constants::BULLET_START,
                    dir.sin() * constants::BULLET_START,
                ),
                self.final_velocity() + na::Vector2::new(
                    dir.cos() * constants::BULLET_VELOCITY,
                    dir.sin() * constants::BULLET_VELOCITY,
                ),
                self.planetype.firepower(),
            ))
        } else {
            None
        }
    }

    pub fn apply_powerup(&mut self, kind: PowerUpKind) {
        // Only allow one weapon at a time
        if kind.is_weapon() {
            self.powerups.retain(|p| !p.kind.is_weapon())
        } else {
            self.powerups.retain(|p| p.kind != kind)
        }
        
        if (kind == PowerUpKind::Health) {
            self.damage_player(-constants::POWERUP_HEALTH_BOOST);
        }

        if (!kind.is_instant()) {
            // Remove duplicates, only allow one weapon
            self.powerups.push(AppliedPowerup::new(kind))
        }
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

    pub fn final_velocity(&self) -> na::Vector2<f32> {
        let mut dx = 0.;
        let mut dy = 0.;

        let has_speed_boost = self.powerups.iter()
            .any(|b| b.kind == PowerUpKind::Afterburner);
        let speed_boost = if(has_speed_boost) {1.8} else {1.};

        let mut speed = self.speed;
        if speed > constants::MAX_SPEED {
            speed = constants::MAX_SPEED;
        }
        if speed < constants::MIN_SPEED {
            speed = constants::MIN_SPEED;
        }

        dx += speed * (self.rotation - std::f32::consts::PI/2.).cos();
        dy += speed * (self.rotation - std::f32::consts::PI/2.).sin();
        na::Vector2::new(dx, dy) * speed_boost
    }

    pub fn max_health(&self) -> i16 {
        self.planetype.health()
    }
}
