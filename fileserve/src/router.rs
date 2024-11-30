use std::io::Cursor;

use shared::image::image_thumb;
use sqlite::Connection;
use tiny_http::{Header, Request, Response};
use url::Url;

use crate::{db, http::parse_request, models::Image};

#[derive(serde::Deserialize)]
pub struct Req {
    start_timestamp: u64,
    stop_timestamp: u64,
}

#[derive(serde::Serialize)]
pub struct ImageListResp {
    images: Vec<Image>,
}

pub fn route_request(mut db: &mut Connection, request: &mut Request) -> Response<Cursor<Vec<u8>>> {
    let json_opt = parse_request(request);
    let mut body = String::new();
    if let Some(json) = json_opt {
        println!("body: {}", json);
        let req: Req = serde_json::from_str(&json).unwrap();
        let images = db::images_since(db, req.start_timestamp, req.stop_timestamp);
        let req_resp = ImageListResp { images };
        body.push_str(&serde_json::to_string(&req_resp).unwrap());
        let content_type = Header::from_bytes("Content-Type", "application/json").unwrap();
        Response::from_string(body).with_header(content_type)
    } else {
        let url = Url::parse(&("http://localhost".to_owned() + request.url())).unwrap();
        let hash_code = url.path();
        let hash = shared::hash::hash_to_u64(&hash_code[1..]);
        let config = shared::CONFIG.get().unwrap();
        let img_bytes = match db::image_exists(db, hash) {
            Some(image) => {
                let new_height = url
                    .query_pairs()
                    .find(|qp| qp.0 == "h")
                    .map(|qp| u32::from_str_radix(&qp.1, 10).unwrap());
                let filename = config.photos_path.clone() + "/" + &image.filename;
                println!("thumbnail processing {:?}", filename);
                thumbnail(&mut db, filename, new_height)
            }
            None => vec![],
        };
        let content_type = Header::from_bytes("Content-Type", "image/jpeg").unwrap();
        Response::from_data(img_bytes).with_header(content_type)
    }
}

fn thumbnail(_db: &mut Connection, filename: String, height: Option<u32>) -> Vec<u8> {
    let file_bytes = std::fs::read(filename).unwrap();
    image_thumb(&file_bytes, height).unwrap()
}
