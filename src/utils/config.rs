use config::{Config as ConfigBuilder, File};
use serde::de::DeserializeOwned;
use std::path::{Path, PathBuf};
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
pub struct Config {
    inner: ConfigBuilder,
}

impl Config {
    // Langkah 1: Membuat config baru dari file
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let path = path.as_ref();
        // Langkah 2: Validasi file exists
        if !path.exists() {
            return Err(ConfigError::FileNotFound(path.to_path_buf()));
        }

        // Langkah 3: Membuat config builder
        let config = ConfigBuilder::builder()
            // Langkah 4: Tambahkan source file
            .add_source(File::from(path))
            // Langkah 5: Build config
            .build()
            // Langkah 6: Handle error
            .map_err(|e| {
                // Cek jika error berupa IO error
                if e.to_string().contains("I/O error") {
                    ConfigError::ReadError {
                        path: path.to_path_buf(),
                        source: std::io::Error::new(std::io::ErrorKind::Other, e.to_string()),
                    }
                } else {
                    ConfigError::Load(e)
                }
            })?;
        // Langkah 7: Return config yang berhasil dibangun
        Ok(Self { inner: config })
    }

    // Langkah 8: Method untuk mengambil value
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Result<T, ConfigError> {
        self.inner.get(key).map_err(|e| match e {
            config::ConfigError::NotFound(_) => ConfigError::KeyNotFound(key.to_string()),
            config::ConfigError::Type { key: k, .. } => {
                ConfigError::TypeMismatch(k.unwrap_or_else(|| key.to_string()))
            }
            _ => ConfigError::Load(e),
        })
    }
}
