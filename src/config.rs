use std::{fs::File, io::Read, path::Path};

use color_eyre::eyre::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    // SMTP stuff
    pub src_email: String,
    pub src_email_auth: String,
    pub smtp_server: String,
    pub dest_email: String,

    // RAWG
    pub rawg_key: String,
    pub platforms: Vec<u8>,
    pub genres: Vec<u8>,

    // Timing
    pub cron: String,
}

impl Config {
    pub fn from_path(path: &Path) -> Result<Config> {
        let mut config_file = File::open(path)?;

        let mut config_contents = Vec::new();

        config_file.read_to_end(&mut config_contents)?;

        toml::from_slice(&config_contents).context("Invalid config syntax")
    }
}
