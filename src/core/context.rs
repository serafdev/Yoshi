use super::Message;
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

/// Context passed to command handlers
#[derive(Clone)]
pub struct Context {
    pub message: Message,
    pub responder: Arc<dyn Responder>,
}

impl Context {
    pub fn new(message: Message, responder: Arc<dyn Responder>) -> Self {
        Self { message, responder }
    }

    /// Reply to the message
    pub async fn reply(&self, content: impl Into<String>) -> Result<()> {
        self.responder.send_message(&self.message.channel_id, content.into()).await
    }

    /// Send a message to a specific channel
    pub async fn send(&self, channel_id: &str, content: impl Into<String>) -> Result<()> {
        self.responder.send_message(channel_id, content.into()).await
    }
}

/// Trait for sending responses back to the platform
#[async_trait]
pub trait Responder: Send + Sync {
    async fn send_message(&self, channel_id: &str, content: String) -> Result<()>;
}
