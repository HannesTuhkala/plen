use enum_map::{enum_map, EnumMap};
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::mixer::Chunk;

use libplen::powerups::PowerUpKind;
use libplen::player::PlaneType;
use libplen::constants;

pub struct Assets<'ttf, 'r> {
    pub font: sdl2::ttf::Font<'ttf, 'r>,

    pub planes: EnumMap<PlaneType, Texture<'r>>,

    pub miniplane: Texture<'r>,
    pub background: Texture<'r>,
    pub minimap_background: Texture<'r>,
    pub minimap_powerup: Texture<'r>,
    pub hurricane: Texture<'r>,

    pub powerups: EnumMap<PowerUpKind, Texture<'r>>,

    pub menu_background: Texture<'r>,
    pub end_background: Texture<'r>,

    pub yeehaw_1: Texture<'r>,
    pub yeehaw_2: Texture<'r>,

    pub smoke: Texture<'r>,
    pub spark: Texture<'r>,
    pub fire: Texture<'r>,
    pub bullet: Texture<'r>,
    pub missile: Texture<'r>,
    pub laser_charge: Texture<'r>,
    pub laser_firing: Texture<'r>,
    pub laser_decay: [Texture<'r>; 3],

    pub achtung_blitzkrieg_engine: Chunk,
    pub el_pollo_romero_engine: Chunk,
    pub howdy_cowboy_engine: Chunk,
    pub suka_blyat_engine: Chunk,
    pub explosion: Chunk,
    pub powerup: Chunk,
    pub gun: Chunk,
    pub laser_fire_sound: Chunk,
    pub laser_charge_sound: Chunk,
}

impl<'ttf, 'r> Assets<'ttf, 'r> {
    pub fn new(texture_creator: &'r TextureCreator<WindowContext>, ttf_context: &'ttf sdl2::ttf::Sdl2TtfContext) -> Assets<'ttf, 'r> {
        let load_tex = |path: &str| {
            let mut tex = texture_creator.load_texture(path)
                .expect(&format!("Could not load {}", path));
            tex.set_blend_mode(sdl2::render::BlendMode::Blend);
            tex
        };

        let mut assets = Assets {
            font: ttf_context.load_font("resources/yoster.ttf", 15)
                .expect("Could not find font!"),
            planes: enum_map! {
                PlaneType::SukaBlyat => load_tex("resources/fishbed.png"),
                PlaneType::AchtungBlitzKrieg => load_tex("resources/messersmitt.png"),
                PlaneType::ElPolloRomero => load_tex("resources/cessna.png"),
                PlaneType::HowdyCowboy => load_tex("resources/jasgripen.png"),
            },
            background: load_tex("resources/background.png"),
            minimap_background: load_tex("resources/minimap.png"),
            minimap_powerup: load_tex("resources/map_powerup.png"),
            miniplane: load_tex("resources/miniplane.png"),
            powerups: enum_map! {
                PowerUpKind::Afterburner => load_tex("resources/powerups/afterburner.png"),
                PowerUpKind::Laser => load_tex("resources/powerups/laser.png"),
                PowerUpKind::Health => load_tex("resources/powerups/heal.png"),
                PowerUpKind::Invincibility => load_tex("resources/powerups/invincibility.png"),
                PowerUpKind::Gun => load_tex("resources/powerups/gun.png"),
                PowerUpKind::Missile => load_tex("resources/powerups/missile.png"),
                PowerUpKind::SlowTime => load_tex("resources/powerups/slowtime.png"),
                PowerUpKind::Invisible => load_tex("resources/powerups/invisible.png"),
            },
            hurricane: load_tex("resources/hurricane.png"),
            bullet: load_tex("resources/bullet.png"),
            menu_background: load_tex("resources/menu_background.png"),
            end_background: load_tex("resources/endscreen.png"),
            yeehaw_1: load_tex("resources/yeehaw.png"),
            yeehaw_2: load_tex("resources/yeehawman.png"),
            smoke: load_tex("resources/smoke.png"),
            spark: load_tex("resources/spark.png"),
            fire: load_tex("resources/small_fire_sprite_orange.png"),
            missile: load_tex("resources/missile.png"),
            laser_charge: load_tex("resources/lasercharge.png"),
            laser_firing: load_tex("resources/laser.png"),
            laser_decay: [
                load_tex("resources/laserdecay_1.png"),
                load_tex("resources/laserdecay_2.png"),
                load_tex("resources/laserdecay_3.png"),
            ],

            achtung_blitzkrieg_engine: Chunk::from_file("resources/audio/achtungblitzkrieg-engine.ogg").unwrap(),
            el_pollo_romero_engine: Chunk::from_file("resources/audio/elpolloromero-engine.ogg").unwrap(),
            howdy_cowboy_engine: Chunk::from_file("resources/audio/howdycowboy-engine.ogg").unwrap(),
            suka_blyat_engine: Chunk::from_file("resources/audio/sukablyat-engine.ogg").unwrap(),
            powerup: Chunk::from_file("resources/audio/powerup.ogg").unwrap(),
            explosion: Chunk::from_file("resources/audio/explosion.ogg").unwrap(),
            gun: Chunk::from_file("resources/audio/gun.ogg").unwrap(),
            laser_fire_sound: Chunk::from_file("resources/audio/laserfire.ogg").unwrap(),
            laser_charge_sound: Chunk::from_file("resources/audio/lasercharge.ogg").unwrap(),
        };

        assets.hurricane.set_alpha_mod((constants::HURRICANE_OPACITY * 255.) as u8);

        // Volume is on a scale from 0 to 128
        assets.achtung_blitzkrieg_engine.set_volume(30);
        assets.el_pollo_romero_engine.set_volume(30);
        assets.howdy_cowboy_engine.set_volume(30);
        assets.suka_blyat_engine.set_volume(30);

        assets
    }
}
