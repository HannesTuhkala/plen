use std::io::prelude::*;
use std::net::TcpStream;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::input::keyboard;

mod player;
mod bullet;
mod gamestate;
mod constants;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct MainState {
    pos_x: f32,
    pos_y: f32,
    server_stream: TcpStream,
    player: player::Player
}

impl MainState {
    fn new(x: f32, y: f32, stream: TcpStream) -> ggez::GameResult<MainState> {
        let s = MainState {
            pos_x: 0.0,
            pos_y: 0.0,
            server_stream: stream,
            player: player::Player::new(0),
        };
        Ok(s)
    }

    fn get_input(ctx: &ggez::Context) -> Option<Direction> {
        if keyboard::is_key_pressed(ctx, event::KeyCode::Up) {
            Some(Direction::Up)
        } else if keyboard::is_key_pressed(ctx, event::KeyCode::Down) {
            Some(Direction::Down)
        } else if keyboard::is_key_pressed(ctx, event::KeyCode::Left) {
            Some(Direction::Left)
        } else if keyboard::is_key_pressed(ctx, event::KeyCode::Right) {
            Some(Direction::Right)
        } else {
            None
        }
    }

    fn wrap_around(pos: na::Point2<f32>) -> na::Point2<f32> {
        let mut new_x = pos.x;
        let mut new_y = pos.y;

        if pos.x > constants::WORLD_SIZE {
            new_x = 0.;
        } else if pos.x < 0. {
            new_x = constants::WORLD_SIZE;
        }

        if pos.y > constants::WORLD_SIZE {
            new_y = 0.;
        } else if pos.y < 0. {
            new_y = constants::WORLD_SIZE;
        }

        na::Point2::new(new_x, new_y)
    }

    fn update_player_position(&mut self, ctx: &mut ggez::Context) {
        let mut dx = 0.;
        let mut dy = 0.;
        match MainState::get_input(ctx) {
            Some(Direction::Up) => dy -= constants::DEFAULT_ACCELERATION,
            Some(Direction::Down) => dy += constants::DEFAULT_ACCELERATION,
            Some(Direction::Left) => dx -= constants::DEFAULT_ACCELERATION,
            Some(Direction::Right) => dx += constants::DEFAULT_ACCELERATION,
            None => ()
        };

        self.player.velocity += na::Vector2::new(dx, dy);
        self.player.position = MainState::wrap_around(
            self.player.position + self.player.velocity);
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.update_player_position(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.1, 0.1, 1.0].into());
        self.player.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult { 
    let mut stream = TcpStream::connect("127.0.0.1:30000")?;
    stream.write("hello".as_bytes())?;

    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;
    let msg = String::from_utf8_lossy(&buffer[..]);
    println!("received msg: {}", msg);

    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(0.0, 0.0, stream)?;
    event::run(ctx, event_loop, state)
}
