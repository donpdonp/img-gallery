use data_encoding::BASE64URL_NOPAD;
use serde::{Serialize, Serializer};

#[derive(serde::Serialize)]
pub struct Image {
    pub filename: String,
    #[serde(serialize_with = "urlsafe")]
    pub hash: u64,
}

fn urlsafe<S: Serializer>(v: &u64, serializer: S) -> Result<S::Ok, S::Error> {
    BASE64URL_NOPAD
        .encode(&v.to_le_bytes().to_vec())
        .serialize(serializer)
}
