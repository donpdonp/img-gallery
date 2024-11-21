use std::io::Cursor;

use sqlite::Connection;
use tiny_http::{Request, Response};

use crate::{db, http::parse_request, models::Image};

#[derive(serde::Deserialize)]
pub struct Req {
    start_timestamp: u64,
}

pub struct ReqResp {
    images: Vec<Image>,
}

pub fn route_request(db: &mut Connection, request: &mut Request) -> Response<Cursor<Vec<u8>>> {
    let json_opt = parse_request(request, db);
    let mut body = String::new();
    if let Some(json) = json_opt {
        println!("body: {}", json);
        let req: Req = serde_json::from_str(&json).unwrap();
        let images = db::images_since(db, req.start_timestamp);
        body.push_str(&serde_json::to_string(&images).unwrap())
    } else {
        let hash = request.url();
        println!("read {}", hash);
    }
    Response::from_string(body)
}
