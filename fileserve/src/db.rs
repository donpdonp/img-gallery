use sqlite::{Connection, State};

use crate::models::Image;

pub fn init() -> Connection {
    let connection = sqlite::open("images.sqlite").unwrap();
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS images (
              filename VARCHAR(255)
            )",
        )
        .unwrap();
    connection
}

pub fn images_since(db: &mut Connection, start_timestamp: u64) -> Vec<Image> {
    let mut images: Vec<Image> = Vec::new();
    let mut statement = db.prepare("select * from images").unwrap();
    while let Ok(State::Row) = statement.next() {
        let img: Image = Image {
            filename: statement.read::<String, _>("filename").unwrap(),
        };
        images.push(img)
    }
    images
}

pub(crate) fn exists(c: &mut Connection, path: &str) -> bool {
    return false;
}

pub(crate) fn insert(c: &mut Connection, path: &str) {
    let mut stmt = c
        .prepare("INSERT INTO images (filename) VALUES (?)")
        .unwrap();
    stmt.bind((1, path)).unwrap();
    stmt.next().unwrap();
}
