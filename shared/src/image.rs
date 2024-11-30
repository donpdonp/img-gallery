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

pub fn image_thumb(bytes: &Vec<u8>, height_opt: Option<u32>) -> Result<Vec<u8>, ImageError> {
    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;
    let dim = img.dimensions();
    let (computed_width, computed_height) = if let Some(height) = height_opt {
        (
            (dim.0 as f32 * (height as f32 / dim.1 as f32)) as u32,
            height,
        )
    } else {
        (dim.0 as u32, dim.1 as u32)
    };
    println!(
        "(requested height {:?}) resize {},{} -> {},{}",
        height_opt, dim.0, dim.1, computed_width, computed_height
    );
    let thumbnail = img.resize(
        computed_width,
        computed_height,
        image::imageops::FilterType::Triangle,
    );
    let mut bytes: Vec<u8> = Vec::new();
    thumbnail.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)?;
    Ok(bytes)
}
