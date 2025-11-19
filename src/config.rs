use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub bot: BotConfig,
    #[serde(default)]
    pub platforms: PlatformsConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BotConfig {
    #[serde(default = "default_command_prefix")]
    pub command_prefix: String,
}

fn default_command_prefix() -> String {
    "!".to_string()
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PlatformsConfig {
    pub discord: Option<DiscordConfig>,
    pub slack: Option<SlackConfig>,
    pub telegram: Option<TelegramConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DiscordConfig {
    pub token: String,
    #[serde(default)]
    pub command_prefix: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SlackConfig {
    pub token: String,
    #[serde(default)]
    pub command_prefix: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TelegramConfig {
    pub token: String,
    #[serde(default = "default_telegram_prefix")]
    pub command_prefix: Option<String>,
}

fn default_telegram_prefix() -> Option<String> {
    Some("/".to_string())
}

impl Config {
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let contents = std::fs::read_to_string(path.as_ref())
            .context("Failed to read config file")?;
        let config: Config = toml::from_str(&contents)
            .context("Failed to parse config file")?;
        Ok(config)
    }

    pub fn from_env() -> Result<Self> {
        // Try to read from default locations
        let paths = vec![
            "config.toml",
            "yoshi.toml",
            "/etc/yoshi/config.toml",
        ];

        for path in paths {
            if Path::new(path).exists() {
                return Self::from_file(path);
            }
        }

        anyhow::bail!("No config file found. Checked: {:?}", paths)
    }
}
