use std::io::{Cursor, Error};

use sqlite::Connection;
use tiny_http::{Request, Response};

use crate::{db, http::parse_request, models::Image};

#[derive(serde::Deserialize)]
pub struct Req {
    start_timestamp: u64,
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
        let images = db::images_since(db, req.start_timestamp);
        let req_resp = ImageListResp { images };
        body.push_str(&serde_json::to_string(&req_resp).unwrap())
    } else {
        let hash_code = request.url();
        let hash = shared::hash::hash_to_u64(&hash_code[1..]);
        match db::image_exists(db, hash) {
            Some(image) => thumbnail(&mut db, image.filename),
            None => vec![],
        };
    }
    Response::from_string(body)
}

fn thumbnail(db: &mut Connection, filename: String) -> Vec<u8> {
    vec![]
}
