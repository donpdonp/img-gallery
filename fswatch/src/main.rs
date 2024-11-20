use fileserve::db;
use fileserve::models::Image;
use highway::HighwayHash;
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
                notify::EventKind::Create(notify::event::CreateKind::File) => {
                    sync(event.paths[0].clone())
                }
                _ => (),
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn sync(path: std::path::PathBuf) {
    let hasher = highway::HighwayHasher::default();
    let hash = hasher.hash64(&std::fs::read(&path).unwrap());
    let mut db = db::init();
    if !fileserve::db::exists(&mut db, &hash.to_string()) {
        println!("{:?} processing {}", path, hash);
        let filename = String::from_utf8(Vec::from(
            path.as_path().file_name().unwrap().as_encoded_bytes(),
        ))
        .unwrap();
        let image = Image { filename, hash };
        fileserve::db::image_insert(&mut db, &image);
    } else {
        println!("{:?} exists", path);
    }
}
