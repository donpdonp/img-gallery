use sqlite::{Connection, State};

use crate::models::Image;

pub fn init() -> Connection {
    let filepath = std::fs::canonicalize("images.sqlite").unwrap();
    println!("sqlite3 {}", filepath.as_os_str().to_str().unwrap());
    let connection = sqlite::open(filepath).unwrap();
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS images (
              hash INTEGER PRIMARY KEY,
              filename VARCHAR(255),
              dim_x INTEGER,
              dim_y INTEGER,
              datetime INTEGER
            )",
        )
        .unwrap();
    connection
}

pub fn images_since(db: &mut Connection, start_timestamp: u64, stop_timestamp: u64) -> Vec<Image> {
    let mut images: Vec<Image> = Vec::new();
    let sql = "select * from images where datetime >= ? and datetime < ?";
    println!(
        "images_since: {} {} {}",
        sql, start_timestamp, stop_timestamp
    );
    let mut stmt = db.prepare(sql).unwrap();
    stmt.bind((1, start_timestamp as i64)).unwrap();
    stmt.bind((2, stop_timestamp as i64)).unwrap();
    while let Ok(State::Row) = stmt.next() {
        let img: Image = Image::from_statement(&stmt);
        images.push(img)
    }
    images
}

pub fn image_exists(db: &mut Connection, hash: u64) -> Option<Image> {
    let mut stmt = db.prepare("SELECT * FROM images WHERE hash = ?").unwrap();
    stmt.bind((1, hash as i64)).unwrap();
    if let Ok(State::Row) = stmt.next() {
        Some(Image::from_statement(&stmt))
    } else {
        None
    }
}

pub fn image_insert(c: &mut Connection, image: &Image) {
    let mut stmt = c
        .prepare(
            "INSERT INTO images (hash, filename, dim_x, dim_y, datetime) VALUES (?, ?, ?, ?, ?)",
        )
        .unwrap();
    stmt.bind((1, image.hash as i64)).unwrap();
    stmt.bind((2, image.filename.as_str())).unwrap();
    stmt.bind((3, image.dim.0 as i64)).unwrap();
    stmt.bind((4, image.dim.1 as i64)).unwrap();
    stmt.bind((5, image.datetime as i64)).unwrap();
    loop {
        match stmt.next() {
            Ok(row) => {
                if row == State::Done {
                    break;
                }
            }
            Err(err) => {
                println!("{:?}", err);
                match err.code {
                    Some(code) => {
                        if code == 19 {
                            println!("hash crash!")
                        }
                    }
                    None => (),
                }
                break;
            }
        }
    }
}
