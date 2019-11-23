use crate::assets::Assets;
use libplen::player;
use libplen::constants;

use ggez;
use ggez::event::{self, EventHandler};
use ggez::event::winit_event::{Event, KeyboardInput, WindowEvent, ElementState};
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::input::keyboard;

use whoami;

const PLANES: [player::PlaneType; 4] = [
    player::PlaneType::SukaBlyat,
    player::PlaneType::HowdyCowboy,
    player::PlaneType::ElPolloRomero,
    player::PlaneType::AchtungBlitzKrieg,
];

const COLORS: [player::Color; 5] = [
    player::Color::Red,
    player::Color::Green,
    player::Color::Blue,
    player::Color::Yellow,
    player::Color::Purple,
];

pub struct MenuState<'a> {
    pub plane: player::PlaneType,
    pub name: String,
    pub color: player::Color,
    pub color_selection: usize,
    pub plane_selection: usize,
    assets: &'a Assets,
}

impl<'a> MenuState<'a> {
    pub fn new(assets: &Assets) -> MenuState {
        MenuState {
            name: String::from(whoami::username()),
            plane: player::PlaneType::SukaBlyat,
            color: player::Color::Red,
            color_selection: 0,
            plane_selection: 0,
            assets: assets,
        }
    }
}

impl<'a> MenuState<'a> {
    fn draw_player_name(&mut self, ctx: &mut ggez::Context, assets: &Assets) {
        let (nx, ny) = constants::NAME_POS;
        let mut text = graphics::Text::new(format!(
            "Helo comrade {}", self.name.clone())
        );
        text.set_font(assets.font, graphics::Scale::uniform(15.));
        graphics::draw(ctx, &text,
                       (na::Point2::new(nx + 10., ny + 10.),)).unwrap();
    }

    fn draw_selected_plane(&mut self, ctx: &mut ggez::Context,
                           assets: &Assets) {
        let sprite = assets.planes[&self.plane].clone();
        let text = self.plane.name();
        let (px, py) = constants::PLANE_SELECTION_POS;
        let mut ggez_text = graphics::Text::new(text);
        ggez_text.set_font(assets.font, graphics::Scale::uniform(15.));
        let background_rect = &graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                px, py,
                constants::PLANE_SELECTION_SIZE*1.25,
                constants::PLANE_SELECTION_SIZE
                ),
            [0., 0., 0., 0.5].into()
        ).unwrap();
        graphics::draw(
            ctx, background_rect,
            (na::Point2::new(0., 0.),)
        ).unwrap();
        graphics::draw(ctx, &ggez_text,
                       (na::Point2::new(px + 10., py + 10.),)).unwrap();
        let mut instruction = graphics::Text::new("click to change plane blyat:");
        instruction.set_font(assets.font, graphics::Scale::uniform(15.));
        graphics::draw(ctx, &instruction,
                       (na::Point2::new(px, py - 20.),)).unwrap();
        graphics::draw(ctx, &sprite,
                       (na::Point2::new(
                               px
                               + constants::PLANE_SELECTION_SIZE/3.
                               - (constants::PLANE_SIZE as f32)*2.,
                               py
                               + constants::PLANE_SELECTION_SIZE/2.
                               - constants::PLANE_SIZE as f32,
                       ),)).unwrap();

        let mut plane_specs = graphics::Text::new(format!(
            "Agility: {}\nFirepower: {}\nAcceleration: {}\nHealth: {}\nResilience: {}",
            self.plane.agility(),
            self.plane.firepower(),
            self.plane.acceleration().trunc(),
            self.plane.health(),
            self.plane.resilience()));
        plane_specs.set_font(assets.font, graphics::Scale::uniform(15.));
        graphics::draw(ctx, &plane_specs,
                       (na::Point2::new(
                               px + constants::PLANE_SELECTION_SIZE/2.4,
                               py + constants::PLANE_SELECTION_SIZE/3.),))
            .unwrap();
    }

    fn draw_selected_color(
        &mut self, ctx: &mut ggez::Context, assets: &Assets
        ) {
        let (cx, cy) = constants::COLOR_SELECTION_POS;
        let background_rect = &graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                cx, cy,
                constants::COLOR_SELECTION_SIZE,
                constants::COLOR_SELECTION_SIZE
                ),
            self.color.rgba().into()
        ).unwrap();
        graphics::draw(
            ctx, background_rect, (na::Point2::new(0., 0.),)).unwrap();
        let mut instruction = graphics::Text::new("click to change color:");
        instruction.set_font(assets.font, graphics::Scale::uniform(15.));
        graphics::draw(ctx, &instruction,
                       (na::Point2::new(cx, cy - 20.),)).unwrap();
    }
}

impl<'a> event::EventHandler for MenuState<'a> {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.plane = PLANES[self.plane_selection].clone();
        self.color = COLORS[self.color_selection].clone();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut ggez::Context,
        keycode: keyboard::KeyCode,
        _keymod: keyboard::KeyMods,
        repeat: bool
    ) {
        if (keycode == keyboard::KeyCode::Return ||
            keycode == keyboard::KeyCode::Space) && !repeat {
            ctx.continuing = false;
        }
    }
    
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.1, 0.1, 1.0].into());
        graphics::draw(ctx, &self.assets.menu_background,
                       (na::Point2::new(0., 0.),)).unwrap();
        self.draw_selected_plane(ctx, self.assets);
        self.draw_selected_color(ctx, self.assets);
        self.draw_player_name(ctx, self.assets);
        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut ggez::Context,
                               _button: ggez::input::mouse::MouseButton,
                               x: f32, y: f32) {
        let (px, py) = constants::PLANE_SELECTION_POS;
        let s = constants::PLANE_SELECTION_SIZE;
        if x > px && x < px + s * 1.25 && y > py && y < py + s {
            self.plane_selection = (self.plane_selection + 1) % 4;
        }

        let (cx, cy) = constants::COLOR_SELECTION_POS;
        let s = constants::COLOR_SELECTION_SIZE;
        if x > cx && x < cx + s && y > cy && y < cy + s {
            self.color_selection = (self.color_selection + 1) % 5;
        }
    }
}
