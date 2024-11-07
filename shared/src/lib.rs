use std::sync::OnceLock;

pub mod config;

pub static CONFIG: OnceLock<config::Config> = OnceLock::new();

#[cfg(test)]
mod tests {}
