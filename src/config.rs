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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_parse_discord() {
        let toml_content = r#"
[bot]
command_prefix = "!"

[platforms.discord]
token = "test_discord_token"
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(toml_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let config = Config::from_file(temp_file.path()).unwrap();

        assert_eq!(config.bot.command_prefix, "!");
        assert!(config.platforms.discord.is_some());
        assert_eq!(
            config.platforms.discord.unwrap().token,
            "test_discord_token"
        );
    }

    #[test]
    fn test_config_parse_all_platforms() {
        let toml_content = r#"
[bot]
command_prefix = "$"

[platforms.discord]
token = "discord_token"
command_prefix = "!"

[platforms.slack]
token = "slack_token"

[platforms.telegram]
token = "telegram_token"
command_prefix = "/"
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(toml_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let config = Config::from_file(temp_file.path()).unwrap();

        assert_eq!(config.bot.command_prefix, "$");
        assert!(config.platforms.discord.is_some());
        assert!(config.platforms.slack.is_some());
        assert!(config.platforms.telegram.is_some());

        let discord = config.platforms.discord.unwrap();
        assert_eq!(discord.command_prefix, Some("!".to_string()));
    }

    #[test]
    fn test_config_default_prefix() {
        let toml_content = r#"
[bot]

[platforms.discord]
token = "test_token"
"#;

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(toml_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let config = Config::from_file(temp_file.path()).unwrap();

        assert_eq!(config.bot.command_prefix, "!");
    }

    #[test]
    fn test_config_invalid_file() {
        let result = Config::from_file("/nonexistent/path/config.toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_config_invalid_toml() {
        let toml_content = "this is not valid toml!!!";

        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(toml_content.as_bytes()).unwrap();
        temp_file.flush().unwrap();

        let result = Config::from_file(temp_file.path());
        assert!(result.is_err());
    }
}
