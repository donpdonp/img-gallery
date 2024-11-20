use sqlite::{Connection, State};

use crate::models::Image;

pub fn init() -> Connection {
    let connection = sqlite::open("images.sqlite").unwrap();
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS images (
              hash INTEGER PRIMARY KEY,
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
            hash: statement.read::<i64, _>("hash").unwrap() as u64,
        };
        images.push(img)
    }
    images
}

pub fn exists(c: &mut Connection, path: &str) -> bool {
    return false;
}

pub fn image_insert(c: &mut Connection, image: &Image) {
    let mut stmt = c
        .prepare("INSERT INTO images (hash, filename) VALUES (?, ?)")
        .unwrap();
    stmt.bind((1, image.hash as i64)).unwrap();
    stmt.bind((2, image.filename.as_str())).unwrap();
    while stmt.next().unwrap() != State::Done {}
}
