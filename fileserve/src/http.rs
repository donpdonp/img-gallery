use sqlite::Connection;
use tiny_http::Request;

pub fn handle_request(request: &mut Request, db: &mut Connection) -> Option<String> {
    println!(
        "received request! method: {:?}, url: {:?}, headers: {:?}",
        request.method(),
        request.url(),
        request.headers()
    );

    if let Some(body_length) = request.body_length() {
        let mut json = String::new();
        request.as_reader().read_to_string(&mut json).unwrap();
        println!("body-length: {}:, ", body_length);
        return Some(json);
    }
    None
}
