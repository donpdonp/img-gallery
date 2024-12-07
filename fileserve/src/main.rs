use std::thread::spawn;

use tiny_http::{Header, Server};

use fileserve::{db, router::route_request};

fn main() -> Result<(), std::io::Error> {
    println!("config {}", shared::CONFIG_FILE);
    shared::CONFIG
        .set(shared::config::load(shared::CONFIG_FILE))
        .unwrap();
    let config = shared::CONFIG.get().unwrap();

    println!("listening {}", &config.listen_address);
    let server = Server::http(&config.listen_address).unwrap();

    // accept connections and process them serially
    for mut request in server.incoming_requests() {
        println!("method: {:?}, url: {:?}", request.method(), request.url(),);

        spawn(|| {
            let mut db = db::init();
            let resp = route_request(&mut db, &mut request);
            let cors_origin = Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap();
            let cors_headers =
                Header::from_bytes("Access-Control-Allow-Headers", "Content-Type").unwrap();
            request
                .respond(resp.with_header(cors_origin).with_header(cors_headers))
                .unwrap();
        });
    }
    Ok(())
}
