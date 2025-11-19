use crate::core::{
    Command, CommandRegistry, Context, Message, MessageContent, Platform, Responder, Author,
};
use anyhow::Result;
use async_trait::async_trait;
use serenity::{
    all::Message as SerenityMessage,
    async_trait as serenity_async_trait,
    client::{Context as SerenityContext, EventHandler},
    model::gateway::Ready,
    prelude::*,
};
use std::sync::Arc;
use tracing::{info, error};

pub struct DiscordPlatform {
    token: String,
    command_prefix: String,
    client: Option<Client>,
}

impl DiscordPlatform {
    pub fn new(token: String, command_prefix: Option<String>) -> Self {
        Self {
            token,
            command_prefix: command_prefix.unwrap_or_else(|| "!".to_string()),
            client: None,
        }
    }
}

#[async_trait]
impl Platform for DiscordPlatform {
    fn name(&self) -> &str {
        "discord"
    }

    async fn start(&mut self, registry: Arc<CommandRegistry>) -> Result<()> {
        info!("Starting Discord platform integration");

        let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;

        let handler = Handler {
            registry,
            command_prefix: self.command_prefix.clone(),
        };

        let client = Client::builder(&self.token, intents)
            .event_handler(handler)
            .await?;

        self.client = Some(client);

        // Start the client (this will block)
        if let Some(client) = &mut self.client {
            client.start().await?;
        }

        Ok(())
    }

    async fn stop(&mut self) -> Result<()> {
        info!("Stopping Discord platform integration");
        // Serenity doesn't provide a clean shutdown method in this version
        // The client will stop when dropped
        self.client = None;
        Ok(())
    }
}

struct Handler {
    registry: Arc<CommandRegistry>,
    command_prefix: String,
}

#[serenity_async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: SerenityContext, msg: SerenityMessage) {
        // Ignore bot messages
        if msg.author.bot {
            return;
        }

        // Check if message starts with command prefix
        if !msg.content.starts_with(&self.command_prefix) {
            return;
        }

        // Remove prefix
        let content = msg.content.strip_prefix(&self.command_prefix).unwrap();

        // Create our platform-agnostic message
        let our_message = Message {
            id: msg.id.to_string(),
            author: Author {
                id: msg.author.id.to_string(),
                name: msg.author.name.clone(),
                is_bot: msg.author.bot,
            },
            content: MessageContent::Text(content.to_string()),
            channel_id: msg.channel_id.to_string(),
            platform: "discord".to_string(),
        };

        // Create responder
        let responder = Arc::new(DiscordResponder {
            ctx: ctx.clone(),
        });

        // Create context
        let context = Context::new(our_message, responder);

        // Execute command
        if let Err(e) = self.registry.execute(content, context).await {
            error!("Error executing command: {}", e);
            if let Err(e) = msg.reply(&ctx.http, format!("Error: {}", e)).await {
                error!("Failed to send error message: {}", e);
            }
        }
    }

    async fn ready(&self, _: SerenityContext, ready: Ready) {
        info!("{} is connected to Discord!", ready.user.name);
    }
}

struct DiscordResponder {
    ctx: SerenityContext,
}

#[async_trait]
impl Responder for DiscordResponder {
    async fn send_message(&self, channel_id: &str, content: String) -> Result<()> {
        let channel_id: u64 = channel_id.parse()?;
        let channel = serenity::all::ChannelId::new(channel_id);
        channel.say(&self.ctx.http, content).await?;
        Ok(())
    }
}
