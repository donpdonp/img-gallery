use figment::{
    providers::{Format, Yaml},
    Figment,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub photos_path: String,
}

pub fn load(filename: &str) -> Config {
    Figment::new()
        .merge(Yaml::file(filename))
        .extract()
        .unwrap()
}
