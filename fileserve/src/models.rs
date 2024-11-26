#[derive(serde::Serialize)]
pub struct Image {
    pub filename: String,
    #[serde(serialize_with = "shared::hash::u64_to_hash")]
    pub hash: u64,
    pub dim: (u32, u32),
    pub datetime: u64,
}

impl Image {
    pub(crate) fn from_statement(statement: &sqlite::Statement<'_>) -> Image {
        let dim = (
            statement.read::<i64, _>("dim_x").unwrap() as u32,
            statement.read::<i64, _>("dim_y").unwrap() as u32,
        );
        Image {
            hash: statement.read::<i64, _>("hash").unwrap() as u64,
            filename: statement.read::<String, _>("filename").unwrap(),
            dim,
            datetime: statement.read::<i64, _>("datetime").unwrap() as u64,
        }
    }
}
