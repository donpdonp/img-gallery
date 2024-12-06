use exif::{Exif, In, Tag};
use fileserve::db;
use fileserve::models::Image;
use highway::{HighwayHash, PortableHash};
use notify::{Event, RecursiveMode, Result, Watcher};
use shared::image::image_dimensions;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use time::{format_description, OffsetDateTime};

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    println!("config {}", shared::CONFIG_FILE);
    shared::CONFIG
        .set(shared::config::load(shared::CONFIG_FILE))
        .unwrap();
    let config = shared::CONFIG.get().unwrap();

    let mut watcher = notify::recommended_watcher(tx)?;
    println!("photos_path {}", &config.photos_path);
    watcher.watch(Path::new(&config.photos_path), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => match event.kind {
                notify::EventKind::Access(notify::event::AccessKind::Close(
                    notify::event::AccessMode::Write,
                )) => sync(event.paths[0].clone()),
                _ => (),
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn sync(path: std::path::PathBuf) {
    let file_bytes = std::fs::read(&path).unwrap();
    let hash = PortableHash::default().hash64(&file_bytes);
    let mut db = db::init();
    if let Some(_image) = fileserve::db::image_exists(&mut db, hash) {
        println!(
            "{:?} (len {}) dupe! hash exists {}",
            path,
            file_bytes.len(),
            hash
        );
    } else {
        println!("{:?} analyzing", path);
        let image = image_analysis(&path, &file_bytes, hash);
        println!("{:?} inserting {} hash", path, hash);
        fileserve::db::image_insert(&mut db, &image);
    }
}

fn image_analysis(path: &PathBuf, bytes: &Vec<u8>, hash: u64) -> Image {
    let filename = String::from_utf8(Vec::from(
        path.as_path().file_name().unwrap().as_encoded_bytes(),
    ))
    .unwrap();
    let dim = image_dimensions(&bytes);
    let exif = exif::Reader::new()
        .read_from_container(&mut Cursor::new(bytes))
        .unwrap();
    let datetime = exif_date_extract(&exif).unix_timestamp() as u64;
    let latlng = exif_latlng_extract(&exif);
    Image {
        filename,
        hash,
        dim,
        datetime,
    }
}

fn exif_latlng_extract(exif: &Exif) -> (i32, i32) {
    // "GPSLatitudeRef" Ascii(["N"])
    let gps_lat_ref_field = exif.get_field(Tag::GPSLatitudeRef, In::PRIMARY).unwrap();
    // "GPSLatitude" Rational([Rational(13/1), Rational(45/1), Rational(461425/10000)])
    let gps_lat_field = exif.get_field(Tag::GPSLatitude, In::PRIMARY).unwrap();
    let lat = if let exif::Value::Rational(gps_lat) = &gps_lat_field.value {
        gps_lat[0].to_f32()
    } else {
        Option::None.unwrap()
    };
    // "GPSLongitudeRef" Ascii(["E"])
    let gps_lon_ref_field = exif.get_field(Tag::GPSLongitudeRef, In::PRIMARY).unwrap();
    // "GPSLongitude" Rational([Rational(100/1), Rational(33/1), Rational(523608/10000)])
    let gps_lon_field = exif.get_field(Tag::GPSLongitude, In::PRIMARY).unwrap();
    println!(
        "{:?} {:?} {:?}",
        gps_lat_field.value,
        gps_lat_field.display_value().to_string(),
        lat
    );
    (0, 0)
}

fn exif_date_extract(exif: &Exif) -> OffsetDateTime {
    // DateTimeOriginal" Ascii(["2024:11:18 12:48:17"])
    let photo_datetime_field = exif.get_field(Tag::DateTime, In::PRIMARY).unwrap();
    // "OffsetTime" Ascii(["+07:00"])
    let photo_timezone_field = exif.get_field(Tag::OffsetTime, In::PRIMARY).unwrap();
    let photo_timezone_str = photo_timezone_field
        .value
        .display_as(Tag::OffsetTime)
        .to_string();
    let photo_timezone_wtf = photo_timezone_str
        .strip_prefix('"') // why
        .unwrap()
        .strip_suffix('"')
        .unwrap();
    let fulldate = format!(
        "{} {}",
        photo_datetime_field.value.display_as(Tag::DateTime),
        photo_timezone_wtf,
    );
    // "2024-11-18 12:48:17 +07:00"
    let format = format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]" ).unwrap();
    OffsetDateTime::parse(&fulldate, &format).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let bytes = vec![];
        let exif = exif::Reader::new()
            .read_from_container(&mut Cursor::new(bytes))
            .unwrap();
        assert_eq!(exif_date_extract(&exif), OffsetDateTime::now_utc());
    }
}
