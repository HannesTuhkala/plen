use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::assets::Assets;
use crate::rendering;
use libplen::player;
use libplen::constants;
use libplen::math::vec2;

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

pub struct MenuState {
    pub plane: player::PlaneType,
    pub name: String,
    pub color: player::Color,
    pub color_selection: usize,
    pub plane_selection: usize,
}

impl MenuState {
    pub fn new() -> MenuState {
        MenuState {
            name: String::new(),
            plane: player::PlaneType::SukaBlyat,
            color: player::Color::Red,
            color_selection: 0,
            plane_selection: 0,
        }
    }
}

impl MenuState {
    fn draw_player_name(&mut self, canvas: &mut Canvas<Window>, assets: &Assets) -> Result<(), String> {
        let (nx, ny) = constants::NAME_POS;
        let text = assets.font.render(&format!("Helo comrade {}", self.name))
            .blended((255, 255, 255))
            .expect("Could not render text");

        let texture_creator = canvas.texture_creator();
        let text_texture = texture_creator.create_texture_from_surface(text).unwrap();

        let res_offset = rendering::calculate_resolution_offset(canvas);
        rendering::draw_texture(
            canvas, &text_texture, vec2(nx + 10., ny + 10.) + res_offset
        )
    }

    fn draw_selected_plane(&mut self, canvas: &mut Canvas<Window>, assets: &Assets) -> Result<(), String> {
        let res_offset = rendering::calculate_resolution_offset(canvas);
        let (px, py) = constants::PLANE_SELECTION_POS;
        let (px, py) = (px + res_offset.x, py + res_offset.y);

        let background_rect = sdl2::rect::Rect::new(
            px as i32,
            py as i32,
            (constants::PLANE_SELECTION_SIZE * 1.25) as u32,
            constants::PLANE_SELECTION_SIZE as u32
        );

        canvas.set_draw_color((0, 0, 0, 128));
        canvas.fill_rect(background_rect)?;

        let texture_creator = canvas.texture_creator();

        let text = assets.font.render(self.plane.name())
            .blended((255, 255, 255))
            .expect("Could not render text");
        let text_texture = texture_creator.create_texture_from_surface(text).unwrap();
        rendering::draw_texture(canvas, &text_texture, vec2(px + 10., py + 10.))?;

        let instruction = assets.font.render("click to change plane blyat:")
            .blended((255, 255, 255))
            .expect("Could not render text");
        let instruction_texture = texture_creator.create_texture_from_surface(instruction).unwrap();

        rendering::draw_texture(canvas, &instruction_texture, vec2(px, py - 20.))?;

        rendering::draw_texture(
            canvas,
            &assets.planes(self.plane),
            vec2(
                px
                    + constants::PLANE_SELECTION_SIZE/3.
                    - (constants::PLANE_SIZE as f32)*2.,
                py
                    + constants::PLANE_SELECTION_SIZE/2.
                    - constants::PLANE_SIZE as f32,
            ))?;

        let specs_string = format!(
            "Top speed: {}\nAgility: {}\nFirepower: {}\nAcceleration: {}\nHealth: {}\nResilience: {}",
            self.plane.max_speed(),
            self.plane.agility(),
            self.plane.firepower(),
            self.plane.acceleration().trunc(),
            self.plane.health(),
            self.plane.resilience()
        );
        let plane_specs = assets.font.render(&specs_string)
            .blended_wrapped((255, 255, 255), 1000)
            .expect("Could not render text");
        let specs_texture = texture_creator.create_texture_from_surface(plane_specs).unwrap();

        rendering::draw_texture(
            canvas,
            &specs_texture,
            vec2(
                px + constants::PLANE_SELECTION_SIZE/2.4,
                py + constants::PLANE_SELECTION_SIZE/3.
            )
        )
    }

    fn draw_selected_color(&mut self, canvas: &mut Canvas<Window>, assets: &Assets) -> Result<(), String> {
        let res_offset = rendering::calculate_resolution_offset(canvas);
        let (cx, cy) = constants::COLOR_SELECTION_POS;
        let (cx, cy) = (cx + res_offset.x, cy + res_offset.y);

        let background_rect = sdl2::rect::Rect::new(
            cx as i32,
            cy as i32,
            constants::COLOR_SELECTION_SIZE as u32,
            constants::COLOR_SELECTION_SIZE as u32
        );
        canvas.set_draw_color(self.color.rgba());
        canvas.fill_rect(background_rect)?;

        let instruction = assets.font.render("click to change color:")
            .blended((255, 255, 255))
            .expect("Could not render text");
        let texture_creator = canvas.texture_creator();
        let instruction_texture = texture_creator.create_texture_from_surface(instruction).unwrap();

        rendering::draw_texture(
            canvas,
            &instruction_texture,
            vec2(cx, cy - 20.)
        )
    }

    pub fn update(&mut self) {
        self.plane = PLANES[self.plane_selection].clone();
        self.color = COLORS[self.color_selection].clone();
    }

    pub fn draw(&mut self, canvas: &mut Canvas<Window>, assets: &Assets) -> Result<(), String> {
        let (width, height) = canvas.logical_size();
        canvas.set_draw_color(constants::MENU_BACKGROUND_COLOR);
        canvas.clear();

        rendering::draw_texture_centered(
            canvas,
            &assets.menu_background,
            vec2(
                width as f32 * 0.5,
                height as f32 * 0.5
            )
        )?;
        self.draw_selected_plane(canvas, assets)?;
        self.draw_selected_color(canvas, assets)?;
        self.draw_player_name(canvas, assets)?;

        canvas.present();
        Ok(())
    }

    pub fn mouse_button_down_event(&mut self, x: f32, y: f32, canvas: &Canvas<Window>) {
        let (px, py) = constants::PLANE_SELECTION_POS;
        let res_offset = rendering::calculate_resolution_offset(canvas);
        let (px, py) = (px + res_offset.x, py + res_offset.y);

        let s = constants::PLANE_SELECTION_SIZE;
        if x > px && x < px + s * 1.25 && y > py && y < py + s {
            self.plane_selection = (self.plane_selection + 1) % 4;
        }

        let (cx, cy) = constants::COLOR_SELECTION_POS;
        let (cx, cy) = (cx + res_offset.x, cy + res_offset.y);
        let s = constants::COLOR_SELECTION_SIZE;
        if x > cx && x < cx + s && y > cy && y < cy + s {
            self.color_selection = (self.color_selection + 1) % 5;
        }
    }
}
