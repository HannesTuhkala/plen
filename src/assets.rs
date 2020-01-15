use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use sdl2::mixer::Chunk;

use libplen::powerups::PowerUpKind;
use libplen::player::PlaneType;
use libplen::constants;

pub struct Assets<'ttf, 'r> {
    pub font: sdl2::ttf::Font<'ttf, 'r>,
    pub plane_textures: PlaneTextures<'r>,
    pub miniplane: Texture<'r>,
    pub background: Texture<'r>,
    pub minimap_background: Texture<'r>,
    pub minimap_powerup: Texture<'r>,
    pub hurricane: Texture<'r>,
    pub powerup_textures: PowerUpTextures<'r>,
    pub bullet: Texture<'r>,
    pub menu_background: Texture<'r>,
    pub end_background: Texture<'r>,
    pub yeehaw_1: Texture<'r>,
    pub yeehaw_2: Texture<'r>,
    pub smoke: Texture<'r>,
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

pub struct PowerUpTextures<'r> {
    pub afterburner: Texture<'r>,
    pub laser: Texture<'r>,
    pub health: Texture<'r>,
    pub invincibility: Texture<'r>,
    pub gun: Texture<'r>,
    pub missile: Texture<'r>,
    pub slow_time: Texture<'r>,
    pub invisible: Texture<'r>,
}

pub struct PlaneTextures<'r> {
    pub suka_blyat: Texture<'r>,
    pub achtung_blitz_krieg: Texture<'r>,
    pub el_pollo_romero: Texture<'r>,
    pub howdy_cowboy: Texture<'r>,
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
            plane_textures: PlaneTextures {
                suka_blyat: load_tex("resources/fishbed.png"),
                achtung_blitz_krieg: load_tex("resources/messersmitt.png"),
                el_pollo_romero: load_tex("resources/cessna.png"),
                howdy_cowboy: load_tex("resources/jasgripen.png"),
            },
            background: load_tex("resources/background.png"),
            minimap_background: load_tex("resources/minimap.png"),
            minimap_powerup: load_tex("resources/map_powerup.png"),
            miniplane: load_tex("resources/miniplane.png"),
            powerup_textures: PowerUpTextures {
                afterburner: load_tex("resources/powerups/afterburner.png"),
                laser: load_tex("resources/powerups/laser.png"),
                health: load_tex("resources/powerups/heal.png"),
                invincibility: load_tex("resources/powerups/invincibility.png"),
                gun: load_tex("resources/powerups/gun.png"),
                missile: load_tex("resources/powerups/missile.png"),
                slow_time: load_tex("resources/powerups/slowtime.png"),
                invisible: load_tex("resources/powerups/invisible.png"),
            },
            hurricane: load_tex("resources/hurricane.png"),
            bullet: load_tex("resources/bullet.png"),
            menu_background: load_tex("resources/menu_background.png"),
            end_background: load_tex("resources/endscreen.png"),
            yeehaw_1: load_tex("resources/yeehaw.png"),
            yeehaw_2: load_tex("resources/yeehawman.png"),
            smoke: load_tex("resources/smoke.png"),
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

    pub fn powerups(&self, kind: PowerUpKind) -> &Texture<'r> {
        match kind {
            PowerUpKind::Afterburner => &self.powerup_textures.afterburner,
            PowerUpKind::Laser => &self.powerup_textures.laser,
            PowerUpKind::Health => &self.powerup_textures.health,
            PowerUpKind::Invincibility => &self.powerup_textures.invincibility,
            PowerUpKind::Gun => &self.powerup_textures.gun,
            PowerUpKind::Missile => &self.powerup_textures.missile,
            PowerUpKind::SlowTime => &self.powerup_textures.slow_time,
            PowerUpKind::Invisible => &self.powerup_textures.invisible,
        }
    }

    pub fn planes(&self, plane_type: PlaneType) -> &Texture<'r> {
        match plane_type {
            PlaneType::SukaBlyat => &self.plane_textures.suka_blyat,
            PlaneType::AchtungBlitzKrieg => &self.plane_textures.achtung_blitz_krieg,
            PlaneType::ElPolloRomero => &self.plane_textures.el_pollo_romero,
            PlaneType::HowdyCowboy => &self.plane_textures.howdy_cowboy,
        }
    }

    pub fn powerups_mut(&mut self, kind: PowerUpKind) -> &mut Texture<'r> {
        match kind {
            PowerUpKind::Afterburner => &mut self.powerup_textures.afterburner,
            PowerUpKind::Laser => &mut self.powerup_textures.laser,
            PowerUpKind::Health => &mut self.powerup_textures.health,
            PowerUpKind::Invincibility => &mut self.powerup_textures.invincibility,
            PowerUpKind::Gun => &mut self.powerup_textures.gun,
            PowerUpKind::Missile => &mut self.powerup_textures.missile,
            PowerUpKind::SlowTime => &mut self.powerup_textures.slow_time,
            PowerUpKind::Invisible => &mut self.powerup_textures.invisible,
        }
    }

    pub fn planes_mut(&mut self, plane_type: PlaneType) -> &mut Texture<'r> {
        match plane_type {
            PlaneType::SukaBlyat => &mut self.plane_textures.suka_blyat,
            PlaneType::AchtungBlitzKrieg => &mut self.plane_textures.achtung_blitz_krieg,
            PlaneType::ElPolloRomero => &mut self.plane_textures.el_pollo_romero,
            PlaneType::HowdyCowboy => &mut self.plane_textures.howdy_cowboy,
        }
    }
}
