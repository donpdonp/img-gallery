#[derive(serde::Serialize)]
pub struct Image {
    pub filename: String,
    #[serde(serialize_with = "shared::hash::u64_to_hash")]
    pub hash: u64,
}
