pub mod db;
pub mod http;
pub mod models;

pub fn sync(path: &str) {
    let mut c = db::init();
    if db::exists(&mut c, path) {
    } else {
        db::insert(&mut c, path);
        println!("inserted {}", path);
    }
}
