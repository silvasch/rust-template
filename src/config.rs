use std::path::{Path, PathBuf};

use crate::prelude::*;

const TARGET: &str = "{{ crate_name }}::config";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Config {
    #[serde(default = "default_foo")]
    pub foo: String,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_home_path = Self::config_home_path()?;
        let _ = std::fs::create_dir_all(&config_home_path);
        let config_file_path = config_home_path.join("config.toml");

        info!(target: TARGET, "loading configuration from '{}'", config_file_path.display());
        let config_file_content = match std::fs::read_to_string(config_file_path) {
            Ok(config_file_content) => config_file_content,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                info!("config file does not exist; using default configuration");
                "".to_string()
            }
            Err(e) => return Err(Error::ConfigRead(e)),
        };

        Ok(toml::from_str(&config_file_content)?)
    }

    fn config_home_path() -> Result<PathBuf> {
        Ok(
            match std::env::var("{{ crate_name | shouty_snake_case }}_CONFIG_HOME") {
                Ok(raw_file_path) => Path::new(&raw_file_path).to_path_buf(),
                Err(_) => match xdg::BaseDirectories::with_prefix("{{ project-name }}") {
                    Ok(base_directories) => base_directories.get_config_home(),
                    Err(_) => return Err(Error::ConfigHome),
                },
            },
        ) // TODO: more functional approach
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            foo: "bar".to_string(),
        }
    }
}

fn default_foo() -> String {
    Config::default().foo
}
