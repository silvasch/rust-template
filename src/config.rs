use std::path::{Path, PathBuf};

use crate::prelude::*;

const TARGET: &str = "{{ crate_name }}::config";

#[derive(Debug)]
pub struct Config {}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        info!(target: TARGET, "trying to load configuration from '{}'", config_path.display());

        Ok(Config {})
    }

    fn config_path() -> Result<PathBuf> {
        Ok(
            match std::env::var("{{ crate_name | shouty_snake_case }}_CONFIG_HOME") {
                Ok(raw_file_path) => Path::new(&raw_file_path).to_path_buf(),
                Err(_) => match xdg::BaseDirectories::with_prefix("{{ project-name }}") {
                    Ok(base_directories) => base_directories.get_config_home(),
                    Err(_) => return Err(Error::ConfigHome),
                },
            },
        )
    }
}
