use fileserve::db;
use fileserve::models::Image;
use highway::{HighwayHash, PortableHash};
use notify::{Event, RecursiveMode, Result, Watcher};
use shared::image::image_dimensions;
use std::path::{Path, PathBuf};
use std::sync::mpsc;

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
    Image {
        filename,
        hash,
        dim,
    }
}
