use ggez;
use ggez::graphics;


pub struct Assets {
    pub cessna: graphics::Image,
    pub background: graphics::Image,
    pub miniplane: graphics::Image,
}

impl Assets {
    
    pub fn new(ctx: &mut ggez::Context) -> Assets {
        Assets {
            cessna: graphics::Image::new(ctx, "/cessna.png").
                expect("Could not find cessna image!"),
            background: graphics::Image::new(ctx, "/background.png").
                expect("Could not find background image!"),
            miniplane: graphics::Image::new(ctx, "/miniplane.png").
                expect("Could not find miniplane image!"),
        }
    }

}
