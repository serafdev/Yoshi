mod core;
mod commands;
mod platforms;
mod config;

use anyhow::Result;
use config::Config;
use core::{CommandRegistry, Platform};
use std::sync::Arc;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("🦖 Starting Yoshi Bot...");

    // Load configuration
    let config = Config::from_env()?;
    info!("Configuration loaded successfully");

    // Create and register all commands
    let mut registry = CommandRegistry::new();
    commands::register_all(&mut registry);
    info!("Registered {} commands", registry.all().len());

    let registry = Arc::new(registry);

    // Start enabled platforms
    let mut handles = vec![];

    #[cfg(feature = "discord")]
    if let Some(discord_config) = config.platforms.discord {
        info!("Discord platform enabled");
        let mut platform = platforms::discord::DiscordPlatform::new(
            discord_config.token,
            discord_config.command_prefix.or(Some(config.bot.command_prefix.clone())),
        );
        let registry_clone = registry.clone();
        let handle = tokio::spawn(async move {
            if let Err(e) = platform.start(registry_clone).await {
                error!("Discord platform error: {}", e);
            }
        });
        handles.push(handle);
    }

    #[cfg(feature = "telegram")]
    if let Some(telegram_config) = config.platforms.telegram {
        info!("Telegram platform enabled");
        let mut platform = platforms::telegram::TelegramPlatform::new(
            telegram_config.token,
            telegram_config.command_prefix,
        );
        let registry_clone = registry.clone();
        let handle = tokio::spawn(async move {
            if let Err(e) = platform.start(registry_clone).await {
                error!("Telegram platform error: {}", e);
            }
        });
        handles.push(handle);
    }

    #[cfg(feature = "slack")]
    if let Some(slack_config) = config.platforms.slack {
        info!("Slack platform enabled");
        let mut platform = platforms::slack::SlackPlatform::new(
            slack_config.token,
            slack_config.command_prefix.or(Some(config.bot.command_prefix.clone())),
        );
        let registry_clone = registry.clone();
        let handle = tokio::spawn(async move {
            if let Err(e) = platform.start(registry_clone).await {
                error!("Slack platform error: {}", e);
            }
        });
        handles.push(handle);
    }

    if handles.is_empty() {
        error!("No platforms enabled! Enable at least one platform in config.toml");
        std::process::exit(1);
    }

    info!("✅ All platforms started. Bot is running!");

    // Wait for all platforms
    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}
