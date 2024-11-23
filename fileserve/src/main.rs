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
    let mut db = db::init();

    // accept connections and process them serially
    for mut request in server.incoming_requests() {
        println!("method: {:?}, url: {:?}", request.method(), request.url(),);

        let resp = route_request(&mut db, &mut request);
        let cors = Header::from_bytes("Access-Control-Allow-Origin", "*").unwrap();
        println!("response bytes {}", resp.data_length().unwrap());
        request.respond(resp.with_header(cors)).unwrap();
    }
    Ok(())
}
