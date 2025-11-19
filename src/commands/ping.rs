use crate::core::{Command, Context};
use anyhow::Result;
use async_trait::async_trait;

pub struct PingCommand;

#[async_trait]
impl Command for PingCommand {
    fn name(&self) -> &str {
        "ping"
    }

    fn description(&self) -> &str {
        "Check if the bot is alive"
    }

    async fn execute(&self, ctx: Context, _args: Vec<String>) -> Result<()> {
        ctx.reply("🏓 Pong!").await?;
        Ok(())
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["pong"]
    }
}
