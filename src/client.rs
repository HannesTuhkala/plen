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
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        update_player_position(ctx, &mut self.player);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.1, 0.1, 1.0].into());
        self.player.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn update_player_position(ctx: &mut ggez::Context,
                          player: &mut player::Player) {
    let mut dx = 0.;
    let mut dy = 0.;
    match get_input(ctx) {
        Some(Direction::Up) => dy -= 1.,
        Some(Direction::Down) => dy += 1.,
        Some(Direction::Left) => dx -= 1.,
        Some(Direction::Right) => dx += 1.,
        None => ()
    };
    
    player.velocity += na::Vector2::new(dx, dy);
    player.position += player.velocity;
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
