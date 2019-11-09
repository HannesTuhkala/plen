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
    scan_angle: f32
}

impl Map {

    pub fn new() -> Map {
        Map {
            scan_angle: 0.
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
        let mut miniplane_sb = spritebatch::SpriteBatch::new(
            assets.miniplane.clone()
        );

        for tile_x in [-1., 0., 1.].iter() {
            for tile_y in [-1., 0., 1.].iter() {
                let offset = na::Vector2::new(
                    tile_x * constants::WORLD_SIZE,
                    tile_y * constants::WORLD_SIZE,
                );
                Map::place_world_at(game_state,
                                    &mut background_sb,
                                    &mut plane_sb,
                                    camera_position,
                                    offset);
            }
        }

        graphics::draw(ctx, &background_sb,
                       graphics::DrawParam::default()
                       .dest(na::Point2::new(0., 0.)))
            .unwrap();
        graphics::draw(ctx, &plane_sb,
                       graphics::DrawParam::default()
                       .dest(na::Point2::new(0., 0.)))
            .unwrap();
        Map::draw_mini_map(game_state, &mut miniplane_sb, ctx, my_id,
                           &my_player, assets);
    }

    fn place_world_at(game_state: &GameState,
                      background_sb: &mut spritebatch::SpriteBatch,
                      plane_sb: &mut spritebatch::SpriteBatch,
                      camera_position: na::Point2<f32>,
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

    fn draw_mini_map(
        game_state: &GameState,
        miniplane_sb: &mut spritebatch::SpriteBatch,
        ctx: &mut ggez::Context,
        my_id: u64,
        my_player: &player::Player,
        assets: &Assets
    ) {
        let mut builder = graphics::MeshBuilder::new();
        Map::add_mini_map_background(&mut builder);
        let mesh = builder.build(ctx).unwrap();
        graphics::draw(ctx, &mesh, (na::Point2::new(
                    constants::WINDOW_SIZE/2. - constants::MINI_MAP_SIZE,
                    constants::WINDOW_SIZE/2. - constants::MINI_MAP_SIZE,
                    ),));
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
