use crate::manifest::Manifest;

use std::fs;

use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub java: Option<JavaConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JavaConfig {
    pub args: Vec<String>,
}

impl Config {
    pub fn path() -> &'static Utf8Path {
        static PATH: OnceCell<Utf8PathBuf> = OnceCell::new();
        let home = Manifest::home_dir();
        PATH.get_or_init(|| home.join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::path();
        let exists = path.exists();

        let config = if exists {
            toml::from_str(&fs::read_to_string(Self::path())?)?
        } else {
            Self { java: None }
        };

        if !exists {
            let content = toml::to_vec(&config)?;
            fs::write(path, content)?;
        }

        Ok(config)
    }
}
