use std::collections::HashMap;
use std::iter::FromIterator;

use ggez;
use ggez::graphics::{Image};

use crate::powerups::PowerUpKind;


pub struct Assets {
    pub cessna: Image,
    pub miniplane: Image,
    pub background: Image,
    pub powerups: HashMap<PowerUpKind, Image>,
    pub bullet: Image,
    pub yeehaw_1: Image,
    pub yeehaw_2: Image,
}

impl Assets {
    pub fn new(ctx: &mut ggez::Context) -> Assets {
        let powerups = HashMap::from_iter(vec!{
            (PowerUpKind::Missile, Image::new(ctx, "/powerups/missile.png")
                .expect("Could not load generic powerup image")),
            (PowerUpKind::Afterburner, Image::new(ctx, "/powerups/afterburner.png")
                .expect("could not load missile powerup asset")),
            (PowerUpKind::Laser, Image::new(ctx, "/powerups/laser.png")
                .expect("could not load missile powerup asset")),
        });
    
        Assets {
            cessna: Image::new(ctx, "/cessna.png").
                expect("Could not find cessna image!"),
            background: Image::new(ctx, "/background.png").
                expect("Could not find background image!"),
            miniplane: Image::new(ctx, "/miniplane.png").
                expect("Could not find miniplane image!"),
            powerups,
            bullet: Image::new(ctx, "/bullet.png").
                expect("Could not find bullet image!"),
            yeehaw_1: Image::new(ctx, "/yeehaw.png").
                expect("Could not find secret 1!"),
            yeehaw_2: Image::new(ctx, "/yeehawman.png").
                expect("Could not find secret 2!"),
        }
    }
}
