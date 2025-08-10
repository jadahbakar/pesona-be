use std::path::{Path, PathBuf};

use config::ConfigBuilder;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to load or parse configuration file")]
    Load(#[from] config::ConfigError),
    #[error("Config key not found: {0}")]
    KeyNotFound(String),
    #[error("Type mismatch for key {0}")]
    TypeMismatch(String),
    #[error("Config file not found: {0}")]
    FileNotFound(PathBuf),
    #[error("IO error while reading {path}: {source}")]
    ReadError {
        path: PathBuf,
        source: std::io::Error,
    },
}

#[derive(Clone, Debug)]
pub struct Config {}

impl Config {
    pub fn builder<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                ConfigError::FileNotFound(path.to_path_buf()) // Menyimpan path asli
            } else {
                ConfigError::Io(e)
            }
        })?;
    }
}
