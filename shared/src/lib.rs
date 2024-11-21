use std::sync::OnceLock;

pub mod config;
pub mod hash;

pub const CONFIG_FILE: &str = "config.yaml";
pub static CONFIG: OnceLock<config::Config> = OnceLock::new();

#[cfg(test)]
mod tests {}
