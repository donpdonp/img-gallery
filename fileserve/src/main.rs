use tiny_http::{Header, Response, Server};

use fileserve::{db, http::handle_request, models::Image};

#[derive(serde::Deserialize)]
struct Req {
    start_timestamp: u64,
}

struct ReqResp {
    images: Vec<Image>,
}

fn main() -> Result<(), std::io::Error> {
    println!("config {}", shared::CONFIG_FILE);
    shared::CONFIG
        .set(shared::config::load(shared::CONFIG_FILE))
        .unwrap();
    let config = shared::CONFIG.get().unwrap();

    println!("listening {}", &config.listen_address);
    let server = Server::http(&config.listen_address).unwrap();
    let mut db = db::init();

    // accept connections and process them serially
    for mut request in server.incoming_requests() {
        println!(
            "method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );

        let json_opt = handle_request(&mut request, &mut db);
        let mut body = String::new();
        if let Some(json) = json_opt {
            let req: Req = serde_json::from_str(&json).unwrap();
            let images = db::images_since(&mut db, req.start_timestamp);
            body.push_str(&serde_json::to_string(&images).unwrap())
        } else {
        }
        let cors = Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap();
        let resp = Response::from_string(body).with_header(cors);
        request.respond(resp).unwrap();
    }
    Ok(())
}
