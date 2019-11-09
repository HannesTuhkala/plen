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
        let background_position = na::Point2::new(
            -camera_position.x,
            -camera_position.y
        );
        let mut background_sb = spritebatch::SpriteBatch::new(
            assets.background.clone()
        );
        background_sb.add(graphics::DrawParam::new()
                          .dest(background_position));
        graphics::draw(ctx, &background_sb, (na::Point2::new(0., 0.),))
            .unwrap();
        my_player.draw(
            ctx,
            na::Point2::new(0.0, 0.0),
            my_player.rotation, assets
        ).unwrap();

        for player in &game_state.players {
            if player.id != my_id {
                let position = na::Point2::new(
                    player.position.x - camera_position.x,
                    player.position.y - camera_position.y,
                );
                player.draw(ctx, position, player.rotation, assets).unwrap();
            }
        }
    }
}
