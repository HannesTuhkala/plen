use crate::player;
use crate::constants;
use crate::gamestate::GameState;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::spritebatch;
use ggez::nalgebra as na;
use ggez::input::keyboard;
use rand::prelude::*;

use crate::assets::Assets;
use crate::powerups::PowerUpKind;
use crate::player::PlaneType;

use std::collections::HashMap;

#[derive(Clone)]
pub struct SmokeParticle {
    alive: bool,
    alpha: f32,
    position: na::Point2<f32>,
}

impl SmokeParticle {
    fn new() -> Self {
        Self {
            alive: false,
            alpha: 0.,
            position: na::Point2::new(0., 0.),
        }
    }
}

pub struct Map {
    scan_angle: f32,
    smoke_particles: Vec<SmokeParticle>,
    smoke_timer: f32,
}

impl Map {
    pub fn new() -> Map {
        Map {
            scan_angle: 0.,
            smoke_particles: vec![SmokeParticle::new(); 200],
            smoke_timer: 0.
        }
    }

    pub fn update_particles(&mut self, delta_time: f32, game_state: &GameState) {
        self.smoke_timer -= delta_time;
        if self.smoke_timer <= 0. {
            let mut rng = rand::thread_rng();
            self.smoke_timer = constants::PARTICLE_SPAWN_RATE;
            for player in &game_state.players {
                let random_offset = na::Vector2::new(
                    (rng.gen::<f32>() - 0.5) * 5.,
                    (rng.gen::<f32>() - 0.5) * 5.,
                );
                for smoke_particle in &mut self.smoke_particles {
                    if !smoke_particle.alive {
                        smoke_particle.alive = true;
                        smoke_particle.alpha = 1.0;
                        smoke_particle.position = player.position + random_offset;
                        break;
                    }
                }
            }
        }

        for smoke_particle in &mut self.smoke_particles {
            if smoke_particle.alive {
                smoke_particle.alpha -= delta_time;
                if smoke_particle.alpha <= 0. {
                    smoke_particle.alive = false;
                }
            }
        }
    }

    pub fn draw(
        &self,
        my_id: u64,
        ctx: &mut ggez::Context,
        camera_position: na::Point2<f32>,
        game_state: &GameState,
        assets: &Assets
    ) {
        let mut background_sb = spritebatch::SpriteBatch::new(
            assets.background.clone()
        );
        let mut plane_sbs = assets.planes.iter()
            .map(|(k, v)| {
                (k.clone(), spritebatch::SpriteBatch::new(v.clone()))
            })
            .collect();
        let mut miniplane_sb = spritebatch::SpriteBatch::new(
            assets.miniplane.clone()
        );

        let mut powerup_sbs = assets.powerups.iter()
            .map(|(k, v)| {
                (k.clone(), spritebatch::SpriteBatch::new(v.clone()))
            })
            .collect();

        let mut bullet_sb = spritebatch::SpriteBatch::new(
            assets.bullet.clone()
        );

        let mut yeehaw_sb = spritebatch::SpriteBatch::new(
            assets.yeehaw_1.clone()
        );

        let mut smoke_sb = spritebatch::SpriteBatch::new(
            assets.smoke.clone()
        );

        for tile_x in [-1., 0., 1.].iter() {
            for tile_y in [-1., 0., 1.].iter() {
                let offset = na::Vector2::new(
                    tile_x * constants::WORLD_SIZE,
                    tile_y * constants::WORLD_SIZE,
                );

                self.place_world_at(
                    game_state,
                    &mut background_sb,
                    &mut plane_sbs,
                    &mut powerup_sbs,
                    &mut bullet_sb,
                    &mut smoke_sb,
                    camera_position,
                    offset
                );
            }
        }

        Self::draw_ui(my_id, game_state, &mut powerup_sbs, &mut yeehaw_sb);

        graphics::draw(ctx, &background_sb, (na::Point2::new(0., 0.),)).unwrap();
        graphics::draw(ctx, &smoke_sb, (na::Point2::new(0., 0.),)).unwrap();
        for sb in plane_sbs.values() {
            graphics::draw(ctx, sb, (na::Point2::new(0., 0.),)).unwrap();
        }

        for sb in powerup_sbs.values() {
            graphics::draw(ctx, sb, (na::Point2::new(0., 0.),)) .unwrap();
        }
        for tile_x in [-1., 0., 1.].iter() {
            for tile_y in [-1., 0., 1.].iter() {
                for player in &game_state.players {
                    let offset = na::Vector2::new(
                        tile_x * constants::WORLD_SIZE,
                        tile_y * constants::WORLD_SIZE,
                    );

                    let position = na::Point2::new(
                        player.position.x - camera_position.x,
                        player.position.y - camera_position.y,
                    ) + offset;

                    let health = player.health as f32;
                    let max_health = player.planetype.health() as f32;
                    const HEALTH_BAR_WIDTH: f32 = 50.;
                    let red_rect = graphics::Rect::new(
                        position.x - HEALTH_BAR_WIDTH / 2.,
                        position.y + 50.,
                        HEALTH_BAR_WIDTH,
                        10.
                    );
                    let green_rect = graphics::Rect::new(
                        position.x - HEALTH_BAR_WIDTH / 2.,
                        position.y + 50.,
                        HEALTH_BAR_WIDTH * health / max_health,
                        10.
                    );
                    let red_mesh = graphics::Mesh::new_rectangle(
                        ctx, graphics::DrawMode::fill(), red_rect, graphics::Color::new(1., 0., 0., 1.)
                    ).unwrap();
                    let green_mesh = graphics::Mesh::new_rectangle(
                        ctx, graphics::DrawMode::fill(), green_rect, graphics::Color::new(0., 1., 0., 1.)
                    ).unwrap();
                    graphics::draw(ctx, &red_mesh, graphics::DrawParam::default()).unwrap();
                    graphics::draw(ctx, &green_mesh, graphics::DrawParam::default()).unwrap();
                }
            }
        }

        graphics::draw(ctx, &bullet_sb, (na::Point2::new(0., 0.),)).unwrap();
        if let Some(my_player) = game_state.get_player_by_id(my_id) {
            Map::draw_mini_map(game_state, &mut miniplane_sb, ctx, &my_player);
        }

        graphics::draw(ctx, &yeehaw_sb, (na::Point2::new(0., 0.),)).unwrap();
    }

    fn place_world_at(
        &self,
        game_state: &GameState,
        background_sb: &mut spritebatch::SpriteBatch,
        plane_sbs: &mut HashMap<PlaneType, spritebatch::SpriteBatch>,
        powerup_sbs: &mut HashMap<PowerUpKind, spritebatch::SpriteBatch>,
        bullet_sb: &mut spritebatch::SpriteBatch,
        smoke_sb: &mut spritebatch::SpriteBatch,
        camera_position: na::Point2<f32>,
        offset: na::Vector2<f32>
    ) {
        let background_position = na::Point2::new(
            -camera_position.x,
            -camera_position.y
        ) + offset;
        background_sb.add(
            graphics::DrawParam::new()
                .dest(background_position)
        );
        for player in &game_state.players {
            let position = na::Point2::new(
                player.position.x - camera_position.x,
                player.position.y - camera_position.y,
            ) + offset;
            plane_sbs.get_mut(&player.planetype).expect("Missing plane asset").add(
                graphics::DrawParam::default()
                    .dest(position)
                    .rotation(player.rotation)
                    .scale(na::Vector2::new(1.0 - player.angular_velocity.abs() / 8., 1.0))
                    .offset(na::Point2::new(0.5, 0.5))
            );
        }

        for bullet in &game_state.bullets {
            let position = na::Point2::new(
                bullet.position.x - camera_position.x,
                bullet.position.y - camera_position.y,
            ) + offset;
            bullet_sb.add(
                graphics::DrawParam::default()
                    .dest(position)
                    .offset(na::Point2::new(0.5, 0.5)));
        }

        for powerup in game_state.powerups.iter() {
            let position = na::Point2::new(
                powerup.position.x - camera_position.x,
                powerup.position.y - camera_position.y,
            ) + offset;
            powerup_sbs.get_mut(&powerup.kind)
                .expect("No powerup asset for this kind")
                .add(graphics::DrawParam::default()
                     .dest(position)
                     .offset(na::Point2::new(0.5, 0.5)));
        }


        for smoke_particle in &self.smoke_particles {
            let position = na::Point2::new(
                smoke_particle.position.x - camera_position.x,
                smoke_particle.position.y - camera_position.y,
            ) + offset;
            if smoke_particle.alive {
                smoke_sb.add(
                    graphics::DrawParam::default()
                        .dest(position)
                        .offset(na::Point2::new(0.5, 0.5))
                        .color(graphics::Color::new(1.0, 1.0, 1.0, smoke_particle.alpha))
                        .scale(na::Vector2::new(0.2, 0.2)));
            }
        }
    }

    fn draw_ui(
        my_id: u64,
        game_state: &GameState,
        powerup_sbs: &mut HashMap<PowerUpKind, spritebatch::SpriteBatch>,
        yeehaw_sb: &mut spritebatch::SpriteBatch,
    ) {
        let mut x_pos = -constants::WINDOW_SIZE/2. + 40.;
        let y_pos = constants::WINDOW_SIZE/2. - 20. - constants::POWERUP_RADIUS as f32;

        game_state.get_player_by_id(my_id)
            .map(|p| {
                for powerup in p.powerups.iter() {
                    powerup_sbs.get_mut(&powerup.kind)
                        .expect("Missing powerup graphics")
                        .add(graphics::DrawParam::default()
                            .dest(na::Point2::new(x_pos, y_pos))
                            .offset(na::Point2::new(0.5, 0.5))
                        );
                    x_pos += constants::POWERUP_RADIUS as f32 * 2.5;
                }
            });

        let position = na::Point2::new(
            constants::WINDOW_SIZE - 480.,
            -constants::WINDOW_SIZE/2.);
        yeehaw_sb.add(
            graphics::DrawParam::default()
                .dest(position)
                .scale(na::Vector2::new(0.3, 0.3)));
    }

    fn draw_mini_map(
        game_state: &GameState,
        miniplane_sb: &mut spritebatch::SpriteBatch,
        ctx: &mut ggez::Context,
        my_player: &player::Player,
    ) {
        let mut builder = graphics::MeshBuilder::new();
        Map::add_mini_map_background(&mut builder);
        let mesh = builder.build(ctx).unwrap();
        graphics::draw(ctx, &mesh, (na::Point2::new(
            constants::WINDOW_SIZE/2. - constants::MINI_MAP_SIZE,
            constants::WINDOW_SIZE/2. - constants::MINI_MAP_SIZE,
        ),)).unwrap();
        let my_pos = my_player.position;
        for tile_x in [-1., 0., 1.].iter() {
            for tile_y in [-1., 0., 1.].iter() {
                for player in &game_state.players {
                    let offset = na::Vector2::new(
                        tile_x * constants::MINI_MAP_SIZE,
                        tile_y * constants::MINI_MAP_SIZE,
                    );
                    let scale = constants::MINI_MAP_SIZE
                        / constants::WORLD_SIZE;
                    let position = na::Point2::new(
                        (player.position.x - my_pos.x)*scale,
                        (player.position.y - my_pos.y)*scale,
                    );
                    let dist = ((position.x + offset.x).powi(2)
                                + (position.y + offset.y).powi(2)).sqrt();
                    if dist <= constants::MINI_MAP_SIZE/2. {
                        miniplane_sb.add(
                            graphics::DrawParam::default()
                                .dest(position + offset)
                                .rotation(player.rotation)
                                .scale(na::Vector2::new(0.5, 0.5))
                                .offset(na::Point2::new(0.5, 0.5))
                        );
                    }
                }
            }
        }
        graphics::draw(
            ctx, miniplane_sb, graphics::DrawParam::default()
                .dest(
                    na::Point2::new(
                        constants::WINDOW_SIZE/2.
                        - constants::MINI_MAP_SIZE/2.,
                        constants::WINDOW_SIZE/2.
                        - constants::MINI_MAP_SIZE/2.
            )))
            .unwrap();
    }

    fn add_mini_map_background(builder: &mut graphics::MeshBuilder) {
        let green_color: graphics::Color = [0., 1., 0., 0.8].into();
        let bg_color: graphics::Color = [0., 0., 0., 0.8].into();

        builder
            .circle( // background
                graphics::DrawMode::fill(),
                na::Point2::new(
                    constants::MINI_MAP_SIZE/2.,
                    constants::MINI_MAP_SIZE/2.,
                ),
                constants::MINI_MAP_SIZE/2.,
                0.1,
                bg_color 
            )
            .circle(
                graphics::DrawMode::stroke(1.),
                na::Point2::new(
                    constants::MINI_MAP_SIZE/2.,
                    constants::MINI_MAP_SIZE/2.,
                ),
                (constants::MINI_MAP_SIZE/2.)*0.9,
                0.1,
                green_color
            )
            .circle(
                graphics::DrawMode::stroke(1.),
                na::Point2::new(
                    constants::MINI_MAP_SIZE/2.,
                    constants::MINI_MAP_SIZE/2.,
                ),
                (constants::MINI_MAP_SIZE/4.)*0.9,
                0.1,
                green_color
            )
            .line( // horizontal line
                &[
                    na::Point2::new(
                        0.,
                        constants::MINI_MAP_SIZE/2.,
                    ),
                    na::Point2::new(
                        constants::MINI_MAP_SIZE,
                        constants::MINI_MAP_SIZE/2.,
                    ),
                ],
                1.,
                green_color
            ).unwrap()
            .line( // vertical line
                &[
                    na::Point2::new(
                        constants::MINI_MAP_SIZE/2.,
                        0.,
                    ),
                    na::Point2::new(
                        constants::MINI_MAP_SIZE/2.,
                        constants::MINI_MAP_SIZE,
                    ),
                ],
                1.,
                green_color
            ).unwrap()
            ;
    }
}
