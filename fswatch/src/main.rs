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
    let datetime = exif_date_extract(bytes).unix_timestamp() as u64;
    Image {
        filename,
        hash,
        dim,
        datetime,
    }
}

fn exif_date_extract(bytes: &Vec<u8>) -> OffsetDateTime {
    let exifreader = exif::Reader::new();
    let exif = exifreader
        .read_from_container(&mut Cursor::new(bytes))
        .unwrap();
    let mut photo_datetime: Option<String> = None;
    let mut photo_timezone: Option<String> = None;
    for f in exif.fields() {
        match f.tag {
            exif::Tag::DateTime => {
                // Ascii(["2024:11:18 12:48:17"])
                photo_datetime = Some(f.display_value().to_string());
            }
            exif::Tag::OffsetTime => {
                // Ascii(["+07:00"])
                photo_timezone = Some(
                    f.display_value()
                        .to_string()
                        .strip_prefix('"')
                        .unwrap()
                        .strip_suffix('"')
                        .unwrap()
                        .to_string(),
                );
            }
            _ => (),
        }
    }
    let fulldate = format!("{} {}", photo_datetime.unwrap(), photo_timezone.unwrap());
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
        assert_eq!(exif_date_extract(&bytes), OffsetDateTime::now_utc());
    }
}
