use std::io::prelude::*;
use std::net::TcpStream;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;

struct MainState {
    pos_x: f32,
    pos_y: f32,
    server_stream: TcpStream,
}

impl MainState {
    fn new(x: f32, y: f32, stream: TcpStream) -> ggez::GameResult<MainState> {
        let s = MainState { pos_x: 0.0, pos_y: 0.0, server_stream: stream };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(self.pos_x, 380.0),
            100.0,
            2.0,
            graphics::WHITE,
        )?;
        graphics::draw(ctx, &circle, (na::Point2::new(0.0, 0.0),))?;

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
