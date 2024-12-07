use std::io::{Cursor, Read};

use multipart::server::{HttpRequest, Multipart, ReadEntry};
use shared::image::image_thumb;
use sqlite::Connection;
use tiny_http::{Header, Method, Request, Response};
use url::Url;

use crate::{db, http::parse_request, models::Image};

#[derive(serde::Deserialize)]
pub struct Req {
    start_timestamp: u64,
    stop_timestamp: u64,
}

#[derive(serde::Serialize)]
pub struct ErrorResp {}

#[derive(serde::Serialize)]
pub struct ImageListResp {
    images: Vec<Image>,
}

pub struct TinyHttpRequest<'s> {
    request: &'s mut Request,
}

impl<'r> HttpRequest for TinyHttpRequest<'r> {
    type Body = &'r mut dyn Read;

    fn multipart_boundary(&self) -> Option<&str> {
        const BOUNDARY: &str = "boundary=";

        let content_type = self
            .request
            .headers()
            .iter()
            .find(|header| header.field.equiv("Content-Type"))
            .unwrap()
            .value
            .as_str();
        let start = content_type.find(BOUNDARY).unwrap() + BOUNDARY.len();
        let end = content_type[start..]
            .find(';')
            .map_or(content_type.len(), |end| start + end);

        Some(&content_type[start..end])
    }

    fn body(self) -> Self::Body {
        self.request.as_reader()
    }
}

pub fn route_request<'r>(
    db: &mut Connection,
    request: &'r mut Request,
) -> Response<Cursor<Vec<u8>>> {
    println!(
        "route: {} {} {:?}",
        request.method(),
        request.url(),
        request.headers()
    );
    match request.method() {
        Method::Post => {
            // route: POST  content-type: multipart/form-data; boundary=4e204ab2-6e27-4f6d-a91d-6367dc6168da
            let headers = request.headers();
            let content_type = headers
                .iter()
                .find(|h| h.field.equiv("content-type"))
                .unwrap();
            let ctv = content_type.value.to_string();
            let ctc = ctv.split(';').collect::<Vec<&str>>()[0];
            if ctc == "multipart/form-data" {
                let trequest = TinyHttpRequest { request }; // container to make multipart-rs happy with local tiny_http
                let body = match Multipart::from_request(trequest) {
                    Ok(multipart) => save_multipart(multipart),
                    Err(req) => format!("multipart err {}", req.request.url()),
                };
                Response::from_string(body)
            } else if ctc == "application/json" {
                let json_opt = parse_request(request);
                if let Some(json) = json_opt {
                    println!("body: {}", json);
                    let req: Req = serde_json::from_str(&json).unwrap();
                    image_gallery(db, req)
                } else {
                    let err_req = serde_json::to_string(&ErrorResp {}).unwrap();
                    Response::from_string(err_req)
                }
            } else {
                Response::from_string("unknown mimetype")
            }
        }
        Method::Get => {
            if request.url() == "/test" {
                return Response::from_string("").with_status_code(200);
            } else {
                thumbnail(db, request)
            }
        }
        Method::Options => Response::from_string("").with_status_code(200),

        _ => Response::from_string("").with_status_code(200),
    }
}

fn save_multipart(multipart: Multipart<&mut dyn Read>) -> String {
    let config = shared::CONFIG.get().unwrap();
    let mut filename: Option<String> = None;
    let mut entry_result = multipart.read_entry();
    loop {
        match entry_result {
            multipart::server::ReadEntryResult::Entry(mut entry) => {
                println!("entry {:?}", entry.headers);
                if *entry.headers.name == *"upfile" {
                    match entry.data.save().with_dir(config.photos_path.clone()) {
                        multipart::server::SaveResult::Full(save_result) => match save_result {
                            multipart::server::save::SavedData::Text(_) => {
                                todo!()
                            }
                            multipart::server::save::SavedData::Bytes(_) => {
                                todo!()
                            }
                            multipart::server::save::SavedData::File(filename_str, _) => {
                                println!("fullsave: {:?}", filename_str);
                                filename = Some(
                                    filename_str.into_os_string().to_string_lossy().into_owned(),
                                );
                            }
                        },
                        multipart::server::SaveResult::Partial(_, _) => todo!(),
                        multipart::server::SaveResult::Error(_) => todo!(),
                    }
                }
                entry_result = entry.next_entry();
            }
            multipart::server::ReadEntryResult::End(_) => break,
            multipart::server::ReadEntryResult::Error(_, _) => break,
        }
    }
    filename.unwrap_or("moar err".to_owned())
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
