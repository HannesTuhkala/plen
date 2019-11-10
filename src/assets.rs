use std::collections::HashMap;
use std::iter::FromIterator;

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
    pub laser_charge: Image,
    pub laser_firing: Image,
    pub laser_decay: [Image; 3],
}

impl Assets {
    pub fn new(ctx: &mut ggez::Context) -> Assets {
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
            laser_charge: Image::new(ctx, "/lasercharge.png")
                .expect("Could not find laser charge image"),
            laser_firing: Image::new(ctx, "/laser.png")
                .expect("Failed to load laser"),
            laser_decay: [
                Image::new(ctx, "/laserdecay_1.png")
                    .expect("Failed to load laser decay 1"),
                Image::new(ctx, "/laserdecay_2.png")
                    .expect("Failed to load laser decay 2"),
                Image::new(ctx, "/laserdecay_3.png")
                    .expect("Failed to load laser decay 3"),
            ]
        }
    }
}
