use image::{GenericImageView, ImageError, ImageReader};

pub fn image_size(filename: String) -> Result<(u32, u32), ImageError> {
    let img = ImageReader::open(filename)?.decode()?;
    Ok(img.dimensions())
}
