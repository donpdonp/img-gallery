use notify::{Event, RecursiveMode, Result, Watcher};
use std::path::Path;
use std::sync::mpsc;

const CONFIG_FILE: &str = "config.yaml";

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    println!("config {}", CONFIG_FILE);
    shared::CONFIG
        .set(shared::config::load(CONFIG_FILE))
        .unwrap();
    let config = shared::CONFIG.get().unwrap();

    // Use recommended_watcher() to automatically select the best implementation
    // for your platform. The `EventHandler` passed to this constructor can be a
    // closure, a `std::sync::mpsc::Sender`, a `crossbeam_channel::Sender`, or
    // another type the trait is implemented for.
    let mut watcher = notify::recommended_watcher(tx)?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    println!("photos_path {}", &config.photos_path);
    watcher.watch(Path::new(&config.photos_path), RecursiveMode::Recursive)?;

    // Block forever, printing out events as they come in
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
    let filename = path.as_path().file_name().unwrap();
    fileserve::sync(filename.to_str().unwrap());
}
