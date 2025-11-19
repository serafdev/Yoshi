use super::{CommandRegistry, Message};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

/// Trait for platform integrations (Discord, Slack, Telegram, etc.)
#[async_trait]
pub trait Platform: Send + Sync {
    /// Platform name
    fn name(&self) -> &str;

    /// Start the platform integration
    async fn start(&mut self, registry: Arc<CommandRegistry>) -> Result<()>;

    /// Stop the platform integration
    async fn stop(&mut self) -> Result<()>;
}

/// Builder for creating platform instances with configuration
pub trait PlatformBuilder: Send + Sync {
    fn build(&self, config: &toml::Value) -> Result<Box<dyn Platform>>;
}
