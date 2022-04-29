extern crate anyhow;
extern crate log;
extern crate regex;
extern crate reqwest;

pub mod templates;

use crate::image::templates::{CardsheetTemplate, SingleCardTemplate};
use crate::models::Card;

use askama::Template;

use anyhow::Result;

use log::{debug, info, warn};

use std::fs::File;
use std::io::Write;

pub fn generate_card_image_html_string(card_info: &Card) -> Result<String> {
    let substituted_template = SingleCardTemplate::new(card_info).render()?;

    debug!("substituted into template: {:?}", substituted_template);

    Ok(substituted_template.to_string())
}

/// Returns the path of the image it generated.
pub fn generate_card_image(card_info: &Card) -> Result<String> {
    let substituted_template_string = generate_card_image_html_string(card_info)?;

    let expected_image_path = format!("runtime/data/cards/images/{}.png", &card_info.id);
    info!("expected image path: {:?}", expected_image_path);

    // Write the substituted HTML into a file
    let substituted_html_path =
        format!("runtime/data/cards/images/templates/{}.html", &card_info.id);
    std::fs::write(&substituted_html_path, &substituted_template_string)?;

    debug!(
        "finished writing substituted html to: {:?}",
        substituted_html_path
    );

    // Spawn off a sub-process for wkhtmltoimage to convert the image.
    generate_image_using_wkhtmltoimage(420, 300, &substituted_html_path, &expected_image_path)?;

    // Once the image is generated, return the path to it.
    Ok(expected_image_path.to_string())
}

pub fn generate_deck_cardsheet_image(deck_name: &str, cards: Vec<Card>) -> Result<String> {
    let expected_image_path = format!("runtime/data/decks/images/{}.png", deck_name);
    let substituted_html_path = format!("runtime/data/decks/images/templates/{}.html", deck_name);
    let number_of_cards: usize = cards.len();

    let substituted_template = CardsheetTemplate {
        cards: cards
            .into_iter()
            .map(|card| SingleCardTemplate::new(&card))
            .collect(),
    }
    .render()?;

    debug!("substituted into template: {:?}", substituted_template);
    info!("expected image path: {:?}", expected_image_path);

    // Write the substituted HTML into a file
    std::fs::write(&substituted_html_path, &substituted_template)?;

    debug!(
        "finished writing substituted html to: {:?}",
        substituted_html_path
    );

    // Spawn off a sub-process for wkhtmltoimage to convert the image.
    generate_image_using_wkhtmltoimage(
        420 * std::cmp::max(
            2,
            number_of_cards / 10 + ((number_of_cards % 10 > 0) as usize),
        ),
        300 * 10,
        &substituted_html_path,
        &expected_image_path,
    )?;

    // Once the image is generated, return the path to it.
    Ok(expected_image_path.to_string())
}

pub fn generate_image_using_wkhtmltoimage(
    height: usize,
    width: usize,
    substituted_html_path: &str,
    output_path: &str,
) -> Result<()> {
    // Spawn off a sub-process for wkhtmltoimage to convert the image.
    let child = std::process::Command::new("./runtime/bin/wkhtmltoimage")
        .args(vec![
            "--height",
            &height.to_string(),
            "--width",
            &width.to_string(),
            "--enable-local-file-access",
            substituted_html_path,
            output_path,
        ])
        .output()?;

    if !child.status.success() {
        use crate::ServerError::FileIOError;
        Err(FileIOError(std::str::from_utf8(&child.stderr)?.to_string()))?
    }

    debug!("wkhtmltoimage returned success for HTML -> PNG");

    Ok(())
}

/// Retrieve the image for the card art using HTTP.
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
            }
            Err(err) => {
                warn!("Could not get image {:?}; error: {:?}", &url, err);
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
