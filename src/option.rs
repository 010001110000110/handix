use std::path::PathBuf;

use serde::Deserialize;

const DEFAULT_CONFIG_FILE_PATH: &str = "./config.toml";
const DEFAULT_HTTP_ADDR: &str = "localhost:1234";

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub struct Opt {
    /// Set the name of the application.
    pub name: String,

    /// Sets the HTTP address and port will use.
    #[serde(default = "default_http_addr")]
    pub http_addr: String,

    /// Set the path to a configuration file that should be used to setup the engine.
    /// Format must be TOML.
    pub config_file_path: Option<PathBuf>,
}

impl Opt {
    pub fn try_build() -> anyhow::Result<(Self, Option<PathBuf>)> {
        let config_file_path = PathBuf::from(DEFAULT_CONFIG_FILE_PATH);

        if let Ok(config) = std::fs::read_to_string(&config_file_path) {
            // Deserialize the contents of the configuration file into an instance of `Opt`
            let opt_from_config = toml::from_str::<Opt>(&config)?;

            // Check if the configuration file contains the `config_file_path` field
            if opt_from_config.config_file_path.is_some() {
                anyhow::bail!("`config_file_path` is not supported in the configuration file");
            }

            return Ok((opt_from_config, Some(config_file_path)));
        } else {
            anyhow::bail!(
                "unable to open or read the {:?} configuration file.",
                config_file_path
            );
        }
    }
}

/// Functions used to get default value for `Opt` fields, needs to be function because of serde's default attribute.

pub fn default_http_addr() -> String {
    DEFAULT_HTTP_ADDR.to_string()
}
