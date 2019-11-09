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

pub struct Map {

}

impl Map {

    pub fn new() -> Map {
        Map {

        }
    }

    pub fn draw(&self, ctx: &mut ggez::Context,
                my_id: u64, game_state: &GameState,
                assets: &Assets) {
        let my_player = game_state.get_player_by_id(my_id)
            .expect("Could not find my own player!");
        let camera_position = my_player.position;
        let mut background_sb = spritebatch::SpriteBatch::new(
            assets.background.clone()
        );
        let mut plane_sb = spritebatch::SpriteBatch::new(
            assets.cessna.clone()
        );

        for tile_x in [-1., 0., 1.].iter() {
            for tile_y in [-1., 0., 1.].iter() {
                let offset = na::Vector2::new(
                    tile_x * constants::WORLD_SIZE,
                    tile_y * constants::WORLD_SIZE,
                );
                Map::place_world_at(ctx,
                                    game_state,
                                    &mut background_sb,
                                    &mut plane_sb,
                                    camera_position,
                                    my_id,
                                    offset);
            }
        }

        graphics::draw(ctx, &background_sb, (na::Point2::new(0., 0.),))
            .unwrap();
        graphics::draw(ctx, &plane_sb, (na::Point2::new(0., 0.),))
            .unwrap();
    }

    fn place_world_at(ctx: &mut ggez::Context,
                      game_state: &GameState,
                      background_sb: &mut spritebatch::SpriteBatch,
                      plane_sb: &mut spritebatch::SpriteBatch,
                      camera_position: na::Point2<f32>,
                      my_id: u64,
                      offset: na::Vector2<f32>) {
        let background_position = na::Point2::new(
            -camera_position.x,
            -camera_position.y
        ) + offset;
        background_sb.add(graphics::DrawParam::new()
                          .dest(background_position));
        for player in &game_state.players {
            let position = na::Point2::new(
                player.position.x - camera_position.x,
                player.position.y - camera_position.y,
            ) + offset;
            plane_sb.add(graphics::DrawParam::default()
                         .dest(position)
                         .rotation(player.rotation)
                         .offset(na::Point2::new(0.5, 0.5)));
        }
    }
}
