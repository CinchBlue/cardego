extern crate cairo;

use cairo::{Context, Format, ImageSurface};
use std::fs::File;

const CARD_FRONT_FILE_PATH: &str = "runtime/data/cards/card_front.png";
const CARD_BACK_FILE_PATH: &str = "runtime/data/cards/card_back.png";

pub fn render(filename: &str) {
    let mut card_front_file = File::create(CARD_FRONT_FILE_PATH)
            .expect(format!("Couldn't open '{}'", CARD_FRONT_FILE_PATH)
                    .as_ref());
    let mut card_front_surface = ImageSurface::create_from_png(
        &mut card_front_file);
    
    let cr = Context::new(&card_front_surface);
    
    // Examples are in 1.0 x 1.0 coordinate space.
    cr.scale(120.0, 120.0);
    
    // Drawing code goes here.
    cr.set_line_width(0.1);
    cr.set_source_rgb(0.0, 0.0, 0.0);
    cr.rectangle(0.25, 0.25, 0.5, 0.5);
    cr.stroke();
    
    let mut file = File::create(filename)
            .expect(format!("Couldn't create '{}'", filename).as_ref());
    
    match card_front_surface.write_to_png(&mut file) {
        Ok(_) => println!("'{}' created", filename),
        Err(_) => println!("'{}' could not be created", filename),
    }
}
