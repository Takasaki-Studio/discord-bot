use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub owner_id: u64,
}

impl Config {
    pub fn get_configs() -> Self {
        Figment::new()
            .merge(Toml::file("config.toml"))
            .join(Env::prefixed("BOT_"))
            .extract()
            .expect("Failed to load configs")
    }
}
