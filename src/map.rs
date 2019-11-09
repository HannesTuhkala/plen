use crate::player;
use crate::constants;
use crate::gamestate::GameState;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::spritebatch;
use ggez::nalgebra as na;
use ggez::input::keyboard;

use crate::assets::Assets;
use crate::powerups::PowerUpKind;

use std::collections::HashMap;

pub struct Map {

}

impl Map {

    pub fn new() -> Map {
        Map {

        }
    }

    pub fn draw(
        &self,
        ctx: &mut ggez::Context,
        camera_position: na::Point2<f32>,
        game_state: &GameState,
        assets: &Assets
    ) {
        let mut background_sb = spritebatch::SpriteBatch::new(
            assets.background.clone()
        );
        let mut plane_sb = spritebatch::SpriteBatch::new(
            assets.cessna.clone()
        );

        let mut powerup_sbs = assets.powerups.iter()
            .map(|(k, v)| {
                (k.clone(), spritebatch::SpriteBatch::new(v.clone()))
            })
            .collect();

        let mut bullet_sb = spritebatch::SpriteBatch::new(
            assets.bullet.clone()
        );

        for tile_x in [-1., 0., 1.].iter() {
            for tile_y in [-1., 0., 1.].iter() {
                let offset = na::Vector2::new(
                    tile_x * constants::WORLD_SIZE,
                    tile_y * constants::WORLD_SIZE,
                );

                Map::place_world_at(
                    game_state,
                    &mut background_sb,
                    &mut plane_sb,
                    &mut powerup_sbs,
                    &mut bullet_sb,
                    camera_position,
                    offset
                );
            }
        }

        Self::draw_ui(my_id, game_state, &mut powerup_sbs);

        graphics::draw(ctx, &background_sb, (na::Point2::new(0., 0.),)).unwrap();
        graphics::draw(ctx, &plane_sb, (na::Point2::new(0., 0.),)).unwrap();

        for sb in powerup_sbs.values() {
            graphics::draw(ctx, sb, (na::Point2::new(0., 0.),)) .unwrap();
        }

        graphics::draw(ctx, &bullet_sb, (na::Point2::new(0., 0.),)).unwrap();
    }

    fn place_world_at(
        game_state: &GameState,
        background_sb: &mut spritebatch::SpriteBatch,
        plane_sb: &mut spritebatch::SpriteBatch,
        powerup_sbs: &mut HashMap<PowerUpKind, spritebatch::SpriteBatch>,
        bullet_sb: &mut spritebatch::SpriteBatch,
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
            plane_sb.add(
                graphics::DrawParam::default()
                    .dest(position)
                    .rotation(player.rotation)
                    .offset(na::Point2::new(0.5, 0.5)));
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
    }

    fn draw_ui(
        my_id: u64,
        game_state: &GameState,
        powerup_sbs: &mut HashMap<PowerUpKind, spritebatch::SpriteBatch>,
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
    }
}
