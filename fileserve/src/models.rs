#[derive(serde::Serialize)]
pub struct Image {
    pub filename: String,
    #[serde(serialize_with = "shared::hash::u64_to_hash")]
    pub hash: u64,
}

impl Image {
    pub(crate) fn from_statement(statement: &sqlite::Statement<'_>) -> Image {
        Image {
            hash: statement.read::<i64, _>("hash").unwrap() as u64,
            filename: statement.read::<String, _>("filename").unwrap(),
        }
    }
}
