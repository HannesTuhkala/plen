use serde_derive::{Serialize, Deserialize};
use rand::prelude::*;
use nalgebra as na;

use crate::constants::{self, PLANE_SIZE, POWERUP_RADIUS, BULLET_RADIUS};
use crate::player::Player;
use crate::projectiles::LaserBeam;
use crate::powerups::{PowerUpKind, PowerUp};
use crate::killfeed::KillFeed;
use crate::hurricane::Hurricane;
use crate::math::wrap_around;
use crate::projectiles::{ProjectileKind, Projectile};
use rand::Rng;

use strum::IntoEnumIterator;

#[derive(Serialize, Deserialize, Clone)]
pub struct GameState {
    pub players: Vec<Player>,
    pub projectiles: Vec<ProjectileKind>,
    pub powerups: Vec<PowerUp>,
    pub lasers: Vec<LaserBeam>,
    pub killfeed: KillFeed,
    pub hurricane: Option<Hurricane>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            players: Vec::new(),
            projectiles: Vec::new(),
            powerups: Vec::new(),
            lasers: Vec::new(),
            killfeed: KillFeed::new(),
            hurricane: None,
        }
    }

    /**
     *  Updates the gamestate and returns
     *  (
     *  vec with player ids that got hit with bullets,
     *  vec with positions where powerups where picked up,
     *  vec with positions where lasers are fired
     *  )
     */
    pub fn update(&mut self, delta: f32)
        -> (Vec<u64>, Vec<(u64, na::Point2<f32>)>, Vec<na::Point2<f32>>) {
        self.maybe_spawn_hurricane(delta);
        self.update_hurricane(delta);
        let hit_powerup_positions = self.handle_powerups();
        let hit_players = self.handle_bullets(delta);
        let fired_laser_positions = self.handle_lasers(delta);
        self.handle_player_collisions(delta);
        self.killfeed.manage_killfeed(delta);
        (hit_players, hit_powerup_positions, fired_laser_positions)
    }

    fn maybe_spawn_hurricane(&mut self, delta: f32) {
        match self.hurricane {
            None => {
                let rand_number = rand::thread_rng().gen_range(0., 1.);
                if rand_number < constants::HURRICANE_PROBABILITY*delta {
                    let xv = rand::thread_rng().gen_range(0., 1.)*constants::HURRICANE_MOVE_SPEED;
                    let yv = rand::thread_rng().gen_range(0., 1.)*constants::HURRICANE_MOVE_SPEED;

                    let xp = rand::thread_rng().gen_range(0., 1.)*constants::WORLD_SIZE;
                    let yp = rand::thread_rng().gen_range(0., 1.)*constants::WORLD_SIZE;

                    let vel = na::Vector2::new(xv, yv);
                    let pos = na::Point2::new(xp, yp);

                    self.hurricane = Some(Hurricane::new(pos, vel));
                }
            }
            _ => ()
        }
    }

    pub fn update_hurricane(&mut self, delta: f32) {
        let mut should_remove_hurricane = false;
        self.hurricane.as_mut().map(|hurricane| {
            hurricane.update(delta);
            should_remove_hurricane = hurricane.is_dead();
        });
        if should_remove_hurricane {
            self.hurricane = None;
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player.clone());
        let msg = player.name + " has joined the game.";
        self.killfeed.add_message(&msg);
    }

    pub fn get_player_by_id(&self, id: u64) -> Option<&Player> {
        for player in &self.players {
            if player.id == id {
                return Some(player);
            }
        }
        None
    }

    pub fn add_bullet(&mut self, projectile: ProjectileKind) {
        self.projectiles.push(ProjectileKind::from(projectile))
    }

    /**
     * Updates the powerups and handles collision detection of them.
     * Returns a vector with (player ids of players who picked up powerups, their positions)
     */
    pub fn handle_powerups(&mut self) -> Vec<(u64, na::Point2<f32>)> {
        let mut new_powerups = self.powerups.clone();
        let mut hit_powerup_positions = vec!();
        for player in &mut self.players {
            new_powerups = new_powerups.into_iter()
                .filter_map(|powerup| {
                    let hit_radius = PLANE_SIZE + POWERUP_RADIUS;
                    if (powerup.position - player.position).norm() < hit_radius as f32 {
                        // Apply the powerup
                        player.apply_powerup(powerup.kind);
                        hit_powerup_positions.push((player.id, player.position));
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
                PowerUp::new(Self::create_powerup(), na::Point2::new(x, y))
            )
        }
        hit_powerup_positions
    }

    fn create_powerup() -> PowerUpKind {
        let max_number: i32 = PowerUpKind::iter().map(|e| e.get_likelihood()).sum();
        let mut rand_number = rand::thread_rng().gen_range(1, max_number);
        let mut powerup = PowerUpKind::Gun;

        for p in PowerUpKind::iter() {
            if rand_number <= 0 {
                return powerup;
            }

            rand_number -= p.get_likelihood();
            powerup = p;
        }

        powerup
    }

    pub fn handle_bullets(&mut self, delta_time: f32) -> Vec<u64> {
        for projectile in &mut self.projectiles {
            projectile.update(&self.players, delta_time, &self.hurricane);
        }

        self.projectiles.retain(
            |projectile| projectile.is_done()
        );

        let mut hit_players: Vec<u64> = Vec::new();
        let hit_radius = PLANE_SIZE * BULLET_RADIUS;
        let mut bullets_to_remove = vec!();

        for projectile in &mut self.projectiles {
            let killer = projectile.get_shooter_name().clone();
            
            for player in &mut self.players {
                let distance = (projectile.get_position() - player.position).norm();
                if distance < hit_radius as f32 && projectile.is_armed() {
                    player.damage_player(projectile.get_damage());
                    if player.has_died() {
                        let msg;
                        
                        if projectile.get_id() == player.id {
                            let msg = player.name.clone() + " killed themselves using a Gun.";
                        } else {
                            let msg = killer.clone() + " killed " + 
                                &player.name + " using a Gun.";
                        }

                        self.killfeed.add_message(&msg);
                    }
                    bullets_to_remove.push(projectile.get_id());
                    hit_players.push(player.id);
                }
            }
        }
        self.projectiles.retain(
            |bullet| !bullets_to_remove.contains(&bullet.get_id())
        );
        hit_players
    }

    /**
     * Returns a vec with positions where lasers are fired.
     */
    pub fn handle_lasers(&mut self, delta: f32) -> Vec<na::Point2<f32>> {
        let mut new_lasers = vec!();
        let mut fired_laser_positions = vec!();
        for player in &self.players {
            player.maybe_get_laser().map(|l| {
                new_lasers.push(l);
                fired_laser_positions.push(player.position);
            });
        }
        self.lasers.append(&mut new_lasers);
        self.lasers.retain(|l| !l.should_be_removed());

        for laser in &mut self.lasers {
            laser.update(delta);

            let killer = laser.owner_name.clone();

            // Check collision
            let direction = na::Vector2::new(
                (laser.angle + std::f32::consts::PI / 2.).cos(),
                (laser.angle + std::f32::consts::PI / 2.).sin()
            );

            let hit_radius = PLANE_SIZE + constants::LASER_RANGE_EXTRA;
            for player in &mut self.players {
                if player.id == laser.owner {
                    continue
                }
                let mut lowest_distance = 100000.;
                for step in 0..100 {
                    let position = laser.position +
                        (-direction * ((step as f32 /100. as f32) * constants::LASER_RANGE));
                    let distance = (wrap_around(position) - wrap_around(player.position)).norm();
                    if distance < lowest_distance {
                        lowest_distance = distance;
                    }

                    // if laser has lived its full life then it should not damage anymore,
                    // even though last phase is shown of the laser
                    if distance < hit_radius as f32 && laser.lifetime > 0. {
                        // bullets_to_remove.push(bullet.id);
                        player.damage_player(laser.damage);

                        if player.has_died() {
                            let msg = killer.clone() + " killed " +
                                &player.name + " using a Laser.";
                            self.killfeed.add_message(&msg);
                        }

                        break;
                    }
                }
            }
        }
        fired_laser_positions
    }

    pub fn handle_player_collisions(&mut self, delta: f32) {
        let mut collided_players: Vec<(u64, String)> = vec!();
        let hit_radius = PLANE_SIZE * 2;

        for p1 in &self.players {
            for p2 in &self.players {
                let distance = (p1.position - p2.position).norm();
                if p1.id != p2.id && distance < hit_radius as f32 {
                    //&& !collided_players.contains(&p1.id) {
                        collided_players.push((p1.id, p2.name));
                }
            }
        }

        for player in &mut self.players {
            player.update_collision_timer(delta);

            for (id, attacker) in &collided_players {
                if player.id == *id && player.time_to_next_collision == 0. {
                    player.damage_player(constants::COLLISION_DAMAGE);
                    
                    if player.has_died() {
                        let msg = format!("{} killed {} by collision.", attacker.clone(), &player.name());
                        self.killfeed.add_message(msg.to_str());
                    }

                    player.time_to_next_collision = constants::COLLISION_GRACE_PERIOD;
                }
            }
        }
    }
}
