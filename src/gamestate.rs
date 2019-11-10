use serde_derive::{Serialize, Deserialize};
use rand::prelude::*;
use nalgebra as na;

use crate::constants::{self, PLANE_SIZE, POWERUP_RADIUS, BULLET_RADIUS};
use crate::player::Player;
use crate::bullet::{LaserBeam, Bullet};
use crate::powerups::{PowerUpKind, PowerUp};
use crate::math::wrap_around;

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub players: Vec<Player>,
    pub bullets: Vec<Bullet>,
    pub powerups: Vec<PowerUp>,
    pub lasers: Vec<LaserBeam>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            players: Vec::new(),
            bullets: Vec::new(),
            powerups: Vec::new(),
            lasers: Vec::new(),
        }
    }

    pub fn update(&mut self, delta: f32) {
        self.handle_powerups();
        self.handle_bullets();
        self.handle_lasers(delta);
        self.handle_player_collisions(delta);
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player)
    }

    pub fn get_player_by_id(&self, id: u64) -> Option<&Player> {
        for player in &self.players {
            if player.id == id {
                return Some(player);
            }
        }
        None
    }

    pub fn add_bullet(&mut self, bullet: Bullet) {
        self.bullets.push(bullet)
    }

    pub fn handle_powerups(&mut self) {
        let mut new_powerups = self.powerups.clone();
        for player in &mut self.players {
            new_powerups = new_powerups.into_iter()
                .filter_map(|powerup| {
                    let hit_radius = PLANE_SIZE + POWERUP_RADIUS;
                    if (powerup.position - player.position).norm() < hit_radius as f32 {
                        // Apply the powerup
                        player.apply_powerup(powerup.kind);
                        None
                    }
                    else {
                        Some(powerup)
                    }
                })
                .collect()
        }
        self.powerups = new_powerups;

        // Create new powerups if there are too few left
        while self.powerups.len() < constants::POWERUP_AMOUNT as usize {
            let x = random::<f32>() * constants::WORLD_SIZE as f32;
            let y = random::<f32>() * constants::WORLD_SIZE as f32;
            self.powerups.push(
                PowerUp::new(random::<PowerUpKind>(), na::Point2::new(x, y))
            )
        }
    }

    pub fn handle_bullets(&mut self) {
        let hit_radius = PLANE_SIZE * BULLET_RADIUS;
        let mut bullets_to_remove = vec!();

        for player in &mut self.players {
            for bullet in &mut self.bullets {
                let distance = (bullet.position - player.position).norm();
                if distance < hit_radius as f32 && bullet.is_armed() {
                    player.damage_player(bullet.damage);
                    bullets_to_remove.push(bullet.id);
                }
            }
        }
        self.bullets.retain(
            |bullet| !bullets_to_remove.contains(&bullet.id)
        )
    }

    pub fn handle_lasers(&mut self, delta: f32) {
        let mut new_lasers = vec!();
        for player in &self.players {
            player.maybe_get_laser().map(|l| {
                new_lasers.push(l);
            });
        }
        self.lasers.append(&mut new_lasers);
        self.lasers.retain(|l| !l.should_be_removed());

        for laser in &mut self.lasers {
            laser.update(delta);

            // Check collision
            let direction = na::Vector2::new(
                (laser.angle + std::f32::consts::PI / 2.).cos(),
                (laser.angle + std::f32::consts::PI / 2.).sin()
            );

            let hit_radius = PLANE_SIZE + 50;
            for player in &mut self.players {
                if player.id == laser.owner {
                    break
                }
                let mut lowest_distance = 100000.;
                for step in 0..100 {
                    let position = laser.position +
                        (-direction * ((step as f32 /100. as f32) * constants::LASER_RANGE));
                    let distance = (wrap_around(position) - wrap_around(player.position)).norm();
                    if distance < lowest_distance {
                        lowest_distance = distance;
                    }
                    if distance < hit_radius as f32 {
                        // bullets_to_remove.push(bullet.id);
                        player.damage_player(laser.damage);
                        break;
                    }
                }
            }
        }
    }

    pub fn handle_player_collisions(&mut self, delta: f32) {
        let mut collided_players = vec!();
        let hit_radius = PLANE_SIZE * 2;

        for p1 in &self.players {
            for p2 in &self.players {
                let distance = (p1.position - p2.position).norm();
                if p1.id != p2.id && distance < hit_radius as f32 &&
                    !collided_players.contains(&p1.id) {
                    collided_players.push(p1.id);
                }
            }
        }

        for player in &mut self.players {
            player.update_collision_timer(delta);

            for id in &collided_players {
                if player.id == *id && player.time_to_next_collision == 0. {
                    player.health -= constants::COLLISION_DAMAGE;
                    player.time_to_next_collision = constants::COLLISION_GRACE_PERIOD;
                }
            }
        }
    }
}
