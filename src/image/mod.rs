extern crate cairo;
extern crate pango;
extern crate pangocairo;
extern crate reqwest;
extern crate anyhow;
extern crate log;
extern crate regex;

pub mod templates;

use crate::models::Card;
use crate::image::templates::{SingleCardTemplate, CardsheetTemplate};

use askama::Template;

use cairo::{ImageSurface, FontSlant, FontWeight};

use anyhow::{Context, Result};

use log::{debug, warn};

use std::fs::{File};
use std::io::{Write};
use self::regex::NoExpand;

pub const CARD_FRONT_FILE_PATH: &str =
    "static/templates/card_front.png";
pub const CARD_BACK_FILE_PATH: &str =
    "static/templates/card_back.png";

pub const CARD_TEMPLATE_HTML_FILE_PATH: &str =
    "static/templates/card.html";

pub fn generate_image_from_html_template(
    card_info: &Card)
    -> Result<String> {
    
    let expected_image_path =
            format!("runtime/data/cards/images/{}.png", &card_info.id);
    
    let substituted_template = SingleCardTemplate::new(card_info).render()?;
    
    debug!("substituted into template: {:?}", substituted_template);
    
    
    debug!("card template file path: {:?}", CARD_TEMPLATE_HTML_FILE_PATH);
    debug!("expected image path: {:?}", expected_image_path);
    
    // Write the substituted HTML into a file
    let substituted_html_path = format!(
        "runtime/data/cards/images/templates/{}.html", &card_info.id);
    std::fs::write(&substituted_html_path, &substituted_template)?;
    
    debug!("finished writing substituted html to: {:?}",
        substituted_html_path);
    
    // Spawn off a sub-process for wkhtmltoimage to convert the image.
    let child = std::process::Command::new("./wkhtmltoimage")
            .args(vec!["--height","1070",
                "--width", "750",
                "--enable-local-file-access",
                &substituted_html_path,
                &expected_image_path])
            .output()?;
    
    if !child.status.success() {
        use crate::ServerError::FileIOError;
        Err(FileIOError(std::str::from_utf8(&child.stderr)?.to_string()))?
    } else {
        debug!("wkhtmltoimage returned success for HTML -> PNG")
    }
    
    
    // Once the image is generated, return the path to it.
    Ok(expected_image_path.to_string())
}

/// Returns the filename that was generated.
pub fn generate_image_from_html_template_old(
    card_info: &Card,
    html_template: &str) -> Result<String> {
    
    // Replace fields in a copy of the HTML template with the card fields.
    let desc = &card_info.desc.replace(r"(Roll", "<br/>(Roll");
    let desc = desc.replace(r"[", "<br/>[");
    
    let template = html_template.clone().to_owned();
    let template = template.replace(r"{{id}}", &card_info.id.to_string());
    let template = template.replace(r"{{cardclass}}", &card_info.cardclass);
    let template = template.replace(r"{{name}}", &card_info.name);
    let template = template.replace(r"{{action}}", &card_info.action);
    let template = template.replace(r"{{speed}}", &card_info.speed);
    let template = template.replace(r"{{image_url}}",
        card_info.image_url.as_ref().unwrap_or(&"".to_string()));
    let template = template.replace(r"{{description}}", &desc);
    
    debug!("substituted into template: {:?}", template);
    
    let expected_image_path =
            format!("runtime/data/cards/images/{}.png", &card_info.id);
    
    debug!("card template file path: {:?}", CARD_TEMPLATE_HTML_FILE_PATH);
    debug!("expected image path: {:?}", expected_image_path);
    
    // Write the substituted HTML into a file
    let substituted_html_path = format!(
        "runtime/data/cards/images/templates/{}.html", &card_info.id);
    std::fs::write(&substituted_html_path, &template)?;
    
    debug!("finished writing substituted html to: {:?}",
        substituted_html_path);
    
    // Spawn off a sub-process for wkhtmltoimage to convert the image.
    let child = std::process::Command::new("./wkhtmltoimage")
            .args(vec!["--height","1070",
                "--width", "750",
                "--enable-local-file-access",
                &substituted_html_path,
                &expected_image_path])
            .output()?;
    
    if !child.status.success() {
        use crate::ServerError::FileIOError;
        Err(FileIOError(std::str::from_utf8(&child.stderr)?.to_string()))?
    } else {
        debug!("wkhtmltoimage returned success for HTML -> PNG")
    }
    
    
    // Once the image is generated, return the path to it.
    Ok(expected_image_path.to_string())
}

pub fn render_surface(card_info: &Card, local_image_filename: Option<String>)
    -> Result<ImageSurface> {
    let (max_w, max_h) = (720.0, 380.0);
    let (target_x, target_y) = (370.0, 370.0);
    
    // Read the card front image data into memory, and initialize a
    // Cairo context with it.
    let mut card_front_file = File::open(CARD_FRONT_FILE_PATH)
            .context(format!("Couldn't open '{}'", CARD_FRONT_FILE_PATH))?;
    let surface = ImageSurface::create_from_png(&mut card_front_file)
            .context(format!("Could not get surface from card front image file:
            '{}'", CARD_FRONT_FILE_PATH))?;
    let mut cxt = cairo::Context::new(&surface);

    // Render the card background from the downloaded file
    if let Some(local_image_filename) = local_image_filename {
        match render_background(&mut cxt, &local_image_filename, card_info.id) {
            Ok(_) => debug!("Generated card background from {:?}",
                local_image_filename),
            Err(err) => warn!("Failed to render card background: {:?}", err),
        }
    };
    
    // Set the color of text + the font family.
    cxt.set_source_rgb(0.0, 0.0, 0.0);
    cxt.select_font_face("Noto Sans", FontSlant::Normal, FontWeight::Normal);
   
    // Pixel coordinates to draw (for the center of the box).
    let (name_x, name_y) = (430.0, 90.0);
    //let (id_x, id_y) = (725.0, 90.0);
    let (id_x, id_y) = (430.0, 145.0);
    let (cardclass_x, cardclass_y) = (93.0, 88.0);
    let (speedaction_x, speedaction_y) = (375.0, 618.0);
    
    // Top-left coordinates for the description box + height/width bounds
    let (desc_x, desc_y) = (59.0, 689.0);
    let (desc_w, desc_h) = (621, 285);
   
    // Draw the id, name, cardclass, speed/action text.
    
    cxt.set_font_size(if card_info.name.len() < 16 { 60.0 } else { 35.0 });
    cxt.select_font_face("Noto Sans", FontSlant::Normal, FontWeight::Bold);
    text_centered(&mut cxt, &card_info.name, name_x, name_y);
    
    cxt.set_font_size(20.0);
    cxt.select_font_face("Noto Sans", FontSlant::Normal, FontWeight::Normal);
    text_centered(
        &mut cxt,
        &format!("(#{:?})", &card_info.id),
        id_x,
        id_y);
    
    cxt.set_font_size(65.0);
    text_centered(&mut cxt, &card_info.cardclass, cardclass_x, cardclass_y);
    
    cxt.set_font_size(40.0);
    text_centered(&mut cxt,
        format!("{} / {}", card_info.speed, card_info.action).as_ref(),
        speedaction_x,
        speedaction_y);
    
    // Use Pango to draw the description text with word wrap.
    cxt.set_font_size(20.0);
    let layout = pangocairo::create_layout(&cxt).unwrap();
    let font_desc = pango::FontDescription::from_string("Noto Serif 30");
    layout.set_text(&card_info.desc);
    layout.set_font_description(Some(&font_desc));
    layout.set_alignment(pango::Alignment::Left);
    layout.set_width(desc_w * 1024);
    layout.set_height(desc_h);
    layout.set_wrap(pango::WrapMode::Word);
    
    cxt.move_to(desc_x, desc_y);
    pangocairo::update_layout(&cxt, &layout);
    pangocairo::show_layout(&cxt, &layout);
    drop(layout);
    
    // Force a draw onto the surface.
    surface.flush();
    
    Ok(surface)
}

fn render_background(
    cxt: &mut cairo::Context,
    local_image_filename: &str,
    card_id: i32)
    -> Result<()>{
    let (max_w, max_h) = (720.0, 380.0);
    let (target_x, target_y) = (370.0, 370.0);
    // Read the background image local file by expected download name.
    let bg_filename = local_image_filename;
    
    //format! ("runtime/data/cards/images/{:?}-art.png", card_info.id);
    let mut bg_file = File::open(&bg_filename).context(format!("Couldn't open '{}'", &bg_filename))?;
    
    // Initialize the Cairo surface + context.
    let bg_surface = ImageSurface::create_from_png(&mut bg_file).context(format!(
        "Could not get surface from background image file: '{}'",
        &bg_filename))?;
    
    // Keep track of the original image dimensions, but as f64 so that we
    // don't need to cast all the time.
    let (bg_w, bg_h) = (
        bg_surface.get_width() as f64,
        bg_surface.get_height() as f64);
    debug!("bg_w, bg_h = {:?}, {:?}", bg_w, bg_h);
    debug!("target_x, target_y = {:?}, {:?}", target_x, target_y);
    
    // We want to keep the aspect ratio of the image, but calculate the
    // scaling ratios to make the image fit inside of the "image box."
    let (mut size_x, mut size_y) = (bg_w, bg_h);
    debug!("preadjusted ratio: {:?}, {:?}", size_x, size_y);
    
    // NOTE: ratio should always be < 1
    if size_x > max_w {
        let ratio = max_w / size_x;
        size_x *= ratio;
        size_y *= ratio;
    }
    if size_y > max_h {
        let ratio = max_h / size_y;
        size_x *= ratio;
        size_y *= ratio;
    }
    debug!("adjusted ratio: {:?}, {:?}", size_x, size_y);
    
    // This is the final scaling transform ratio.
    let (scale_x, scale_y) = (size_x / bg_w, size_y / bg_h);
    debug!("scaling: {:?}, {:?}", scale_x, scale_y);
    
    // Calculate the final top-left coordinates of the image after
    // the scaling factor is applied.
    debug!("preadjusted target: {:?}, {:?}", target_x, target_y);
    let (target_x, target_y) = (
        (target_x - size_x / 2.0) / scale_x,
        (target_y - size_y / 2.0) / scale_y
    );
    debug!("adjusted target: {:?}, {:?}", target_x, target_y);
    
    // Render the background image, while making sure to push/pop the
    // context state to remove the scaling off of future writes.
    cxt.save();
    
    // Scale first so that when we set the source surface at the xy
    // coordinates, it will be in the correct place.
    cxt.scale(scale_x, scale_y);
    
    // Render the image.
    cxt.set_source_surface(&bg_surface, target_x, target_y);
    cxt.paint();
    cxt.restore();
    
    Ok(())
}

// Draw centered text at the coordinates.
fn text_centered(cxt: &mut cairo::Context, text: &str, x: f64, y: f64) {
    let extents = cxt.text_extents(text);
    let (x, y) =
            (x-(extents.width/2.0 + extents.x_bearing),
                y-(extents.height/2.0 + extents.y_bearing));
    
    cxt.move_to(x, y);
    cxt.show_text(text);
}

pub async fn retrieve_image(url: &str, card_id: i32) -> anyhow::Result<String> {
    let url = reqwest::Url::parse(url)?;
    
    debug!("parsed image url {:?}", &url);
    
    let fname = format!("runtime/data/cards/images/{:?}-art.png", card_id);
    let mut dest = File::create(&fname)?;
    
    if url.scheme() == "file" {
        let filename = &url.path()[1..];
        
        debug!("reading from local file {:?}", &filename);
        let content = std::fs::read(filename)?;
        
        debug!("writing to {:?}", fname);
        dest.write_all(&content[..])?;
    } else {
        debug!("request from url: {}", url);
    
        let response = match reqwest::get(url.clone()).await {
            Ok(res) => {
                debug!("Found successful response");
                res
            },
            Err(err) => {
                warn!("Could not get image {:?}; error: {:?}",
                    &url,
                    err);
                Err(err)?
            }
        };
    
        debug!("response: {:?}", &response);
    
        let mut content = response.bytes().await?;
        
        debug!("found content: {:?}", content);
    
        debug!("writing to {:?}", fname);
        dest.write_all(&mut content)?;
    };
    
    dest.flush()?;
    
    debug!("flushed to {:?}", fname);
    
    Ok(fname.clone())
}
