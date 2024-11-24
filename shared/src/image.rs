use std::io::Cursor;

use image::{GenericImageView, ImageError, ImageReader};

pub fn image_dimensions(bytes: &Vec<u8>) -> (u32, u32) {
    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap();
    img.dimensions()
}

pub fn image_thumb(bytes: &Vec<u8>) -> Result<Vec<u8>, ImageError> {
    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;
    let dim = img.dimensions();
    let thumbnail = img.resize(
        dim.0 / 10,
        dim.1 / 10,
        image::imageops::FilterType::Triangle,
    );
    let mut bytes: Vec<u8> = Vec::new();
    thumbnail.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)?;
    Ok(bytes)
}
