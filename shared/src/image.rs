use std::io::Cursor;

use image::{GenericImageView, ImageError, ImageReader};

pub fn image_thumb(filename: String) -> Result<Vec<u8>, ImageError> {
    let img = ImageReader::open(filename)?.decode()?;
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
