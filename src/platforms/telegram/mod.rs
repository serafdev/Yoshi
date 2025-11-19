use crate::core::{
    Command, CommandRegistry, Context, Message, MessageContent, Platform, Responder, Author,
};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::{info, error};
use teloxide::prelude::*;

pub struct TelegramPlatform {
    token: String,
    command_prefix: String,
}

impl TelegramPlatform {
    pub fn new(token: String, command_prefix: Option<String>) -> Self {
        Self {
            token,
            command_prefix: command_prefix.unwrap_or_else(|| "/".to_string()),
        }
    }
}

#[async_trait]
impl Platform for TelegramPlatform {
    fn name(&self) -> &str {
        "telegram"
    }

    async fn start(&mut self, registry: Arc<CommandRegistry>) -> Result<()> {
        info!("Starting Telegram platform integration");

        let bot = Bot::new(&self.token);
        let command_prefix = self.command_prefix.clone();

        teloxide::repl(bot, move |bot: Bot, msg: teloxide::types::Message| {
            let registry = registry.clone();
            let command_prefix = command_prefix.clone();
            async move {
                if let Some(text) = msg.text() {
                    if text.starts_with(&command_prefix) {
                        let content = text.strip_prefix(&command_prefix).unwrap();

                        let our_message = Message {
                            id: msg.id.to_string(),
                            author: Author {
                                id: msg.from.as_ref().map(|u| u.id.to_string()).unwrap_or_default(),
                                name: msg.from.as_ref().map(|u| u.first_name.clone()).unwrap_or_default(),
                                is_bot: msg.from.as_ref().map(|u| u.is_bot).unwrap_or(false),
                            },
                            content: MessageContent::Text(content.to_string()),
                            channel_id: msg.chat.id.to_string(),
                            platform: "telegram".to_string(),
                        };

                        let responder = Arc::new(TelegramResponder {
                            bot: bot.clone(),
                            chat_id: msg.chat.id,
                        });

                        let context = Context::new(our_message, responder);

                        if let Err(e) = registry.execute(content, context).await {
                            error!("Error executing command: {}", e);
                            let _ = bot.send_message(msg.chat.id, format!("Error: {}", e)).await;
                        }
                    }
                }
                respond(())
            }
        })
        .await;

        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        info!("Stopping Telegram platform integration");
        Ok(())
    }
}

struct TelegramResponder {
    bot: Bot,
    chat_id: teloxide::types::ChatId,
}

#[async_trait]
impl Responder for TelegramResponder {
    async fn send_message(&self, _channel_id: &str, content: String) -> Result<()> {
        self.bot.send_message(self.chat_id, content).await?;
        Ok(())
    }
}
