use sqlite::Connection;
use tiny_http::Request;

pub fn parse_request(request: &mut Request, db: &mut Connection) -> Option<String> {
    if let Some(_) = request.body_length() {
        let mut json = String::new();
        request.as_reader().read_to_string(&mut json).unwrap();
        return Some(json);
    }
    None
}
