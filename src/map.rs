use crate::player;
use crate::constants;
use crate::gamestate::GameState;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::spritebatch;
use ggez::nalgebra as na;
use ggez::input::keyboard;


pub struct Map {
    background_image: graphics::Image
}

impl Map {

    pub fn new(ctx: &mut ggez::Context) -> Map {
        Map {
            background_image: graphics::Image::new(
                                  ctx, "/background.png").unwrap(),
        }
    }

    pub fn draw(ctx: &mut ggez::Context, my_id: u64, gamestate: &GameState) {
        let my_player = gamestate.get_player_by_id(my_id)
            .expect("Could not find my own player!");
        let camera_position = my_player.position;
        let background_position = na::Point2::new(
            camera_position.x - constants::WORLD_SIZE,
            camera_position.y - constants::WORLD_SIZE
        );
    }
}
