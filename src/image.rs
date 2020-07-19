extern crate cairo;
extern crate pango;
extern crate pangocairo;

use crate::models::Card;

use cairo::{Context, ImageSurface, FontSlant, FontWeight};

use std::fs::File;

pub const CARD_FRONT_FILE_PATH: &str =
    "runtime/data/cards/images/templates/card_front.png";
pub const CARD_BACK_FILE_PATH: &str =
    "runtime/data/cards/images/templates/card_back.png";

pub fn render(card_info: &Card) -> ImageSurface {
    // Read the card front image data into memory, and initialize a
    // Cairo context with it.
    let mut card_front_file = File::open(CARD_FRONT_FILE_PATH)
            .expect(format!("Couldn't open '{}'", CARD_FRONT_FILE_PATH)
                    .as_ref());
    let surface = ImageSurface::create_from_png(&mut card_front_file)
            .expect(format!("Could not get surface from card front image file: \
            '{}'", CARD_FRONT_FILE_PATH).as_ref());
    let mut cr = Context::new(&surface);
    
    // Set the color of text + the font family.
    cr.set_source_rgb(0.0, 0.0, 0.0);
    cr.select_font_face("Noto Sans", FontSlant::Normal, FontWeight::Normal);
   
    // Pixel coordinates to draw (for the center of the box).
    let (name_x, name_y) = (430.0, 89.0);
    let (cardclass_x, cardclass_y) = (93.0, 88.0);
    let (speedaction_x, speedaction_y) = (375.0, 618.0);
    
    // Top-left coordinates for the description box + height/width bounds
    let (desc_x, desc_y) = (59.0, 689.0);
    let (desc_w, desc_h) = (621, 285);
   
    // Draw the name, cardclass, speed/action text.
    
    cr.set_font_size(60.0);
    text_centered(&mut cr, &card_info.name, name_x, name_y);
    
    cr.set_font_size(65.0);
    text_centered(&mut cr, &card_info.cardclass, cardclass_x, cardclass_y);
    
    cr.set_font_size(40.0);
    text_centered(&mut cr,
        format!("{} / {}", card_info.speed, card_info.action).as_ref(),
        speedaction_x,
        speedaction_y);
    
    // Use Pango to draw the description text with word wrap.
    cr.set_font_size(20.0);
    let layout = pangocairo::create_layout(&cr).unwrap();
    let font_desc = pango::FontDescription::from_string("Noto Serif 30");
    layout.set_text(&card_info.desc);
    layout.set_font_description(Some(&font_desc));
    layout.set_alignment(pango::Alignment::Left);
    layout.set_width(desc_w * 1024);
    layout.set_height(desc_h);
    layout.set_wrap(pango::WrapMode::Word);
    
    cr.move_to(desc_x, desc_y);
    pangocairo::update_layout(&cr, &layout);
    pangocairo::show_layout(&cr, &layout);
   
    // Force a draw onto the surface.
    surface.flush();
    
    surface
}

// Draw centered text at the coordinates.
fn text_centered(cr: &mut Context, text: &str, x: f64, y: f64) {
    let extents = cr.text_extents(text);
    let (x, y) =
            (x-(extents.width/2.0 + extents .x_bearing),
                y-(extents.height/2.0 + extents.y_bearing));
    
    cr.move_to(x, y);
    cr.show_text(text);
}
