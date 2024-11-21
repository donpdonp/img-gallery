use data_encoding::BASE64URL_NOPAD;
use serde::{Serialize, Serializer};

pub fn hash_to_u64(hash_code: &str) -> u64 {
    let u64_bytes = BASE64URL_NOPAD.decode(hash_code.as_bytes()).unwrap();
    u64::from_le_bytes(u64_bytes[0..8].try_into().unwrap())
}

pub fn u64_to_hash<S: Serializer>(v: &u64, serializer: S) -> Result<S::Ok, S::Error> {
    BASE64URL_NOPAD
        .encode(&v.to_le_bytes().to_vec())
        .serialize(serializer)
}
