use std::sync::OnceLock;

pub mod config;

pub const CONFIG_FILE: &str = "config.yaml";
pub static CONFIG: OnceLock<config::Config> = OnceLock::new();

#[cfg(test)]
mod tests {}
