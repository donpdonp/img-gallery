use fileserve::db;
use fileserve::models::Image;
use highway::{HighwayHash, PortableHash};
use notify::{Event, RecursiveMode, Result, Watcher};
use std::path::Path;
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
    let mut file = std::fs::File::open(&path).unwrap();
    let mut hasher = PortableHash::default();
    let bytes_copied = std::io::copy(&mut file, &mut hasher).unwrap();
    let hash = hasher.finalize64();
    let mut db = db::init();
    if let Some(image) = fileserve::db::image_exists(&mut db, hash) {
        println!(
            "{:?} (len {}) dupe! hash exists {}",
            path, bytes_copied, hash
        );
    } else {
        println!("{:?} inserting {} hash", path, hash);
        let filename = String::from_utf8(Vec::from(
            path.as_path().file_name().unwrap().as_encoded_bytes(),
        ))
        .unwrap();
        let image = Image { filename, hash };
        fileserve::db::image_insert(&mut db, &image);
    }
}
