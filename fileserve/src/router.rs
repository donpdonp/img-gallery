use std::io::Cursor;

use shared::image::image_thumb;
use sqlite::Connection;
use tiny_http::{Header, HeaderField, Method, Request, Response};
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

pub fn route_request(db: &mut Connection, request: &mut Request) -> Response<Cursor<Vec<u8>>> {
    if request.method() == &Method::Post {
        println!("{:?}", request.headers());
        let content_type_header = HeaderField::from_bytes("content-type").unwrap();
        let content_type = request
            .headers()
            .iter()
            .find(|h| h.field == content_type_header)
            .unwrap();
        println!(
            "route: {} content-type {}",
            request.method(),
            content_type.value
        );
        if content_type.value == "application/json" {}
    }
    let json_opt = parse_request(request);
    if let Some(json) = json_opt {
        println!("body: {}", json);
        let req: Req = serde_json::from_str(&json).unwrap();
        image_gallery(db, req)
    } else {
        thumbnail(db, request)
    }
}

fn image_gallery(db: &mut Connection, req: Req) -> Response<Cursor<Vec<u8>>> {
    let images = db::images_since(db, req.start_timestamp, req.stop_timestamp);
    let req_resp = ImageListResp { images };
    let json = serde_json::to_string(&req_resp).unwrap();
    let content_type = Header::from_bytes("Content-Type", "application/json").unwrap();
    Response::from_string(json).with_header(content_type)
}

fn thumbnail(db: &mut Connection, request: &mut Request) -> Response<Cursor<Vec<u8>>> {
    let url = Url::parse(&("http://localhost".to_owned() + request.url())).expect("bad url");
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
            let file_bytes = std::fs::read(filename).unwrap();
            image_thumb(&file_bytes, new_height).unwrap()
        }
        None => vec![],
    };
    let content_type = Header::from_bytes("Content-Type", "image/jpeg").unwrap();
    Response::from_data(img_bytes).with_header(content_type)
}
