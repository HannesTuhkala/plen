mod assets;
mod map;
mod menu;
mod rendering;

use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Instant;

use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::render::BlendMode;
use sdl2::keyboard::{Keycode, Scancode};

use libplen::messages::{
    ClientMessage,
    ClientInput,
    MessageReader,
    ServerMessage,
    SoundEffect
};
use libplen::gamestate;
use libplen::constants;
use libplen::hurricane;
use libplen::math::{Vec2, vec2};
use assets::Assets;
use menu::MenuState;

fn send_client_message(msg: &ClientMessage, stream: &mut TcpStream) {
    let data = bincode::serialize(msg).expect("Failed to encode message");
    let length = data.len() as u16;
    stream.write(&length.to_be_bytes())
        .expect("Failed to send message length to server");
    stream.write(&data)
        .expect("Failed to send message to server");
}

#[derive(PartialEq)]
enum StateResult { Continue, GotoNext }

struct MainState {
    my_id: u64,
    camera_position: Vec2,
    game_state: gamestate::GameState,
    map: map::Map,
    last_time: Instant,
    powerup_rotation: f32,
    hit_effect_timer: f32,
    dead: bool,
}

impl MainState {
    fn new(my_id: u64) -> MainState {
        MainState {
            my_id,
            camera_position: vec2(0., 0.),
            game_state: gamestate::GameState::new(),
            map: map::Map::new(),
            last_time: Instant::now(),
            powerup_rotation: 0.,
            hit_effect_timer: 0.,
            dead: false,
        }
    }

    fn update(
        &mut self,
        assets: &Assets,
        server_reader: &mut MessageReader,
        keyboard_state: &sdl2::keyboard::KeyboardState
    ) -> StateResult {
        self.update_hit_sequence();

        let elapsed = self.last_time.elapsed();
        self.last_time = Instant::now();
        let dt_duration = std::time::Duration::from_millis(1000 / 60);
        if elapsed < dt_duration {
            std::thread::sleep(dt_duration - elapsed);
        }

        server_reader.fetch_bytes().unwrap();

        for message in server_reader.iter() {
            match bincode::deserialize(&message).unwrap() {
                ServerMessage::AssignId(_) => {panic!("Got new ID after intialisation")}
                ServerMessage::GameState(state) => {
                    self.game_state = state
                },
                ServerMessage::PlaySound(sound, pos) => {
                    fn play_sound(soundeffect: &sdl2::mixer::Chunk) {
                        if let Err(e) = sdl2::mixer::Channel::all().play(
                            soundeffect, 0
                        ) {
                            println!("SDL mixer error: {}", e);
                        }
                    }

                    match sound {
                        SoundEffect::Powerup => {
                            play_sound(&assets.powerup);
                        }
                        SoundEffect::Gun => {
                            play_sound(&assets.gun);
                        }
                        SoundEffect::Explosion => {
                            play_sound(&assets.explosion);
                            self.map.add_explosion(pos);
                        }
                        SoundEffect::LaserCharge => {
                            play_sound(&assets.laser_charge_sound);
                        }
                        SoundEffect::LaserFire => {
                            play_sound(&assets.laser_fire_sound);
                        }
                    }
                }
                ServerMessage::YouDied => {
                    self.dead = true;
                }
                ServerMessage::PlayerHit(id) => {
                    // TODO handle if it's someone elses id, for example
                    // for sound effects and stuff
                    if id == self.my_id {
                        self.start_hit_sequence();
                    }
                }
            }
        }

        let mut input = ClientInput::new();
        if keyboard_state.is_scancode_pressed(Scancode::W) {
            input.y_input += 1.0;
        }
        if keyboard_state.is_scancode_pressed(Scancode::S) {
            input.y_input -= 1.0;
        }

        if keyboard_state.is_scancode_pressed(Scancode::A) {
            input.x_input -= 1.0;
        } 
        if keyboard_state.is_scancode_pressed(Scancode::D) {
            input.x_input += 1.0;
        }

        if self.dead && keyboard_state.is_scancode_pressed(Scancode::Return) {
            return StateResult::GotoNext;
        }

        self.map.update(elapsed.as_secs_f32(), &self.game_state, self.my_id);

        input.shooting = keyboard_state.is_scancode_pressed(Scancode::Space);
        input.activating_powerup = keyboard_state.is_scancode_pressed(Scancode::E);
        let input_message = ClientMessage::Input(input);
        send_client_message(&input_message, &mut server_reader.stream);

        self.powerup_rotation += constants::POWERUP_SPEED * elapsed.as_secs_f32();

        StateResult::Continue
    }

    fn draw(&mut self, canvas: &mut Canvas<Window>, assets: &mut Assets) -> Result<(), String> {
        if let Some(my_player) = self.game_state.get_player_by_id(self.my_id) {
            self.camera_position = my_player.position;
        }

        self.map.draw(
            self.my_id,
            canvas,
            self.camera_position,
            &self.game_state,
            assets,
            self.powerup_rotation,
            self.hit_effect_timer,
            &self.game_state.hurricane
        )?;

        if self.dead {
            let (width, height) = canvas.logical_size();
            rendering::draw_texture_centered(
                canvas,
                &assets.end_background,
                vec2(width as f32 * 0.5, height as f32 * 0.3)
            ).unwrap();
        }

        Ok(())
    }
    
    fn start_hit_sequence(&mut self) {
        self.hit_effect_timer = constants::HIT_SEQUENCE_AMOUNT;
    }

    /**
     * Updates the hit sequence timer. Returns true if the hit effect timer is 0,
     * otherwise false.
     */
    fn update_hit_sequence(&mut self) -> bool {
        if self.hit_effect_timer <= 0. {
            self.hit_effect_timer = 0.;
            true
        } else {
            self.hit_effect_timer -= constants::HIT_SEQUENCE_RATE;
            false
        }
    }
}

pub fn main() -> Result<(), String> {
    let host = std::env::var("SERVER")
        .unwrap_or(String::from("localhost:4444"));
    let stream = TcpStream::connect(host).expect("Could not connect to server");
    println!("Connected to server");

    stream.set_nonblocking(true).expect("Could not set socket as nonblocking");
    let mut reader = MessageReader::new(stream);

    let msg = loop {
        reader.fetch_bytes().unwrap();
        if let Some(msg) = reader.iter().next() {
            break bincode::deserialize(&msg).unwrap();
        }
    };

    let my_id = if let ServerMessage::AssignId(id) = msg {
        println!("Received the id {}", id);
        id
    } else {
        panic!("Expected to get an id from server")
    };

    let sdl = sdl2::init().expect("Could not initialize SDL");
    let video_subsystem = sdl.video().expect("Could not initialize SDL video");

    let window = video_subsystem
        .window("plen", constants::WINDOW_SIZE as u32, constants::WINDOW_SIZE as u32)
        .resizable()
        .build()
        .expect("Could not create window");

    let mut canvas = window.into_canvas().build().expect("Could not create canvas");
    canvas.set_blend_mode(BlendMode::Blend);
    let texture_creator = canvas.texture_creator();

    let _audio = sdl.audio().expect("Could not initialize SDL audio");
    let frequency = 44_100;
    let format = sdl2::mixer::AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = sdl2::mixer::DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size).expect("Could not open SDL mixer audio");
    let _mixer_context = sdl2::mixer::init(
        sdl2::mixer::InitFlag::OGG
    ).expect("Could not initialize SDL mixer");

    // Allows 64 sounds to play simultaneously
    sdl2::mixer::allocate_channels(64);

    let ttf_context = sdl2::ttf::init().expect("Could not initialize SDL ttf");

    let mut assets = Assets::new(&texture_creator, &ttf_context);

    let mut color_selection = 0;
    let mut plane_selection = 0;
    let mut name = whoami::username();

    let mut event_pump = sdl.event_pump().expect("Could not get event pump");

    'mainloop: loop {
        let menu_state = &mut MenuState::new();

        video_subsystem.text_input().start();
        menu_state.color_selection = color_selection;
        menu_state.plane_selection = plane_selection;
        menu_state.name = name;

        'menuloop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit{..} => break 'mainloop,
                    Event::KeyDown {keycode: Some(kc), ..} => {
                        match kc {
                            Keycode::Return => {
                                break 'menuloop;
                            }
                            Keycode::Backspace => {
                                menu_state.name.pop();
                            }
                            _ => {}
                        }
                    }
                    Event::MouseButtonDown {x, y, ..} => {
                        menu_state.mouse_button_down_event(x as f32, y as f32, &canvas);
                    }
                    Event::TextInput {text, ..} => {
                        if menu_state.name.chars().count() < 20 {
                            menu_state.name += &text;
                        }
                    }
                    _ => {}
                }
            }
            rendering::setup_coordinates(&mut canvas)?;

            // Ignore all messages so we don't freeze the server
            reader.fetch_bytes().unwrap();
            for _ in reader.iter() {
            }

            menu_state.update();

            menu_state.draw(&mut canvas, &assets).unwrap();
        }
        video_subsystem.text_input().stop();

        color_selection = menu_state.color_selection;
        plane_selection = menu_state.plane_selection;
        name = menu_state.name.clone();

        send_client_message(
            &ClientMessage::JoinGame { 
                name: menu_state.name.clone(),
                plane: menu_state.plane.clone(),
                color: menu_state.color.clone()
            },
            &mut reader.stream
        );

        let main_state = &mut MainState::new(my_id);
        'gameloop: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit{..} => break 'mainloop,
                    _ => {}
                }
            }
            rendering::setup_coordinates(&mut canvas)?;

            canvas.set_draw_color(sdl2::pixels::Color::RGB(25, 25, 25));
            canvas.clear();

            let state_result =
                main_state.update(&assets, &mut reader, &event_pump.keyboard_state());
            main_state.draw(&mut canvas, &mut assets).unwrap();

            canvas.present();

            if state_result == StateResult::GotoNext {
                break 'gameloop;
            }
        }
    }

    Ok(())
}
