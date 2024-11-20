#[derive(serde::Serialize)]
pub struct Image {
    pub filename: String,
    pub hash: u64,
}
