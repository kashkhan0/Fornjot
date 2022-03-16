use std::path::PathBuf;

use anyhow::Context as _;
use figment::{
    providers::{Env, Format as _, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub default_path: PathBuf,
    pub default_model: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self, anyhow::Error> {
        Figment::new()
            .merge(Toml::file("fj.toml"))
            .merge(Env::prefixed("FJ_"))
            .extract()
            .context("Error loading configuration")
    }
}