use std::collections::HashMap;
use std::iter::FromIterator;
use std::io::BufReader;
use std::fs::File;
use std::path::PathBuf;

use ggez;
use ggez::graphics::{Image};

use crate::powerups::PowerUpKind;
use crate::player::PlaneType;


pub struct Assets {
    pub planes: HashMap<PlaneType, Image>,
    pub miniplane: Image,
    pub background: Image,
    pub powerups: HashMap<PowerUpKind, Image>,
    pub bullet: Image,
    pub menu_background: Image,
    pub yeehaw_1: Image,
    pub yeehaw_2: Image,
    pub smoke: Image,

    pub achtung_blitzkrieg_engine: rodio::Decoder<BufReader<File>>,
    pub el_pollo_romero_engine: rodio::Decoder<BufReader<File>>,
    pub howdy_cowboy_engine: rodio::Decoder<BufReader<File>>,
    pub suka_blyat_engine: rodio::Source::Buffered<Item = f32>,
    pub explosion: rodio::Decoder<BufReader<File>>,
    pub powerup: rodio::Decoder<BufReader<File>>,
    pub gun: rodio::Decoder<BufReader<File>>,
}

impl Assets {
    pub fn new(ctx: &mut ggez::Context, resource_dir: &PathBuf) -> Assets {
        let powerups = HashMap::from_iter(vec!{
            (PowerUpKind::Missile, Image::new(ctx, "/powerups/missile.png")
                .expect("Could not load generic powerup image")),
            (PowerUpKind::Afterburner, Image::new(ctx, "/powerups/afterburner.png")
                .expect("Could not load missile powerup asset")),
            (PowerUpKind::Laser, Image::new(ctx, "/powerups/laser.png")
                .expect("Could not load laser powerup asset")),
            (PowerUpKind::Health, Image::new(ctx, "/powerups/heal.png")
                .expect("Could not load health powerup asset")),
            (PowerUpKind::Invincibility, Image::new(ctx, "/powerups/invincibility.png")
                .expect("Could not load invincibility powerup asset")),
            (PowerUpKind::Gun, Image::new(ctx, "/powerups/gun.png")
                .expect("Could not load gun powerup asset")),
        });

        let planes = HashMap::from_iter(vec!{
            (PlaneType::SukaBlyat, Image::new(ctx, "/fishbed.png")
                .expect("Failed to load fishbed")),
            (PlaneType::AchtungBlitzKrieg, Image::new(ctx, "/messersmitt.png")
                .expect("Failed to load messersmitt")),
            (PlaneType::ElPolloRomero, Image::new(ctx, "/cessna.png")
                .expect("Failed to load spanish")),
            (PlaneType::HowdyCowboy, Image::new(ctx, "/jasgripen.png")
                .expect("Failed to load jasgipen")),
        });
    
        Assets {
            planes,
            background: Image::new(ctx, "/background.png")
                .expect("Could not find background image!"),
            miniplane: Image::new(ctx, "/miniplane.png")
                .expect("Could not find miniplane image!"),
            powerups,
            bullet: Image::new(ctx, "/bullet.png")
                .expect("Could not find bullet image!"),
            menu_background: Image::new(ctx, "/menu_background.png").
                expect("Could not find bullet image!"),
            yeehaw_1: Image::new(ctx, "/yeehaw.png").
                expect("Could not find secret 1!"),
            yeehaw_2: Image::new(ctx, "/yeehawman.png").
                expect("Could not find secret 2!"),
            smoke: Image::new(ctx, "/smoke.png")
                .expect("Could not find smoke image"),

            achtung_blitzkrieg_engine: rodio::Decoder::new(BufReader::new(
                    Self::read_audio(
                        resource_dir,
                        "achtungblitzkrieg-engine.ogg"
                        )
                    )).unwrap(),
            el_pollo_romero_engine: rodio::Decoder::new(BufReader::new(
                    Self::read_audio(
                        resource_dir,
                        "elpolloromero-engine.ogg"
                        )
                    )).unwrap(),
            howdy_cowboy_engine: rodio::Decoder::new(BufReader::new(
                    Self::read_audio(
                        resource_dir,
                        "howdycowboy-engine.ogg"
                        )
                    )).unwrap(),
            suka_blyat_engine: BufReader::new(
                    Self::read_audio(
                        resource_dir,
                        "sukablyat-engine.ogg"
                        )
                    ),
            powerup: rodio::Decoder::new(BufReader::new(
                    Self::read_audio(
                        resource_dir,
                        "powerup.ogg"
                        )
                    )).unwrap(),
            explosion: rodio::Decoder::new(BufReader::new(
                    Self::read_audio(
                        resource_dir,
                        "explosion.ogg"
                        )
                    )).unwrap(),
            gun: rodio::Decoder::new(BufReader::new(
                    Self::read_audio(
                        resource_dir,
                        "gun.ogg"
                        )
                    )).unwrap(),
        }
    }

    fn read_audio(resource_dir: &PathBuf, name: &str) -> File {
        let mut path = resource_dir.clone();
        path.push("audio");
        path.push(name);
        File::open(path.clone()).expect(
            &format!("Could not find audio file {:?}", path)
        )
    }
}
