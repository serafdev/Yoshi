use crate::core::{Command, Context};
use anyhow::Result;
use async_trait::async_trait;
use std::time::Instant;

static START_TIME: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

pub struct UptimeCommand;

impl UptimeCommand {
    pub fn init() {
        START_TIME.get_or_init(Instant::now);
    }
}

#[async_trait]
impl Command for UptimeCommand {
    fn name(&self) -> &str {
        "uptime"
    }

    fn description(&self) -> &str {
        "Show bot uptime"
    }

    async fn execute(&self, ctx: Context, _args: Vec<String>) -> Result<()> {
        if let Some(start) = START_TIME.get() {
            let uptime = start.elapsed();
            let seconds = uptime.as_secs();
            let days = seconds / 86400;
            let hours = (seconds % 86400) / 3600;
            let minutes = (seconds % 3600) / 60;
            let secs = seconds % 60;

            let response = format!(
                "⏱️ **Bot Uptime**: {}d {}h {}m {}s",
                days, hours, minutes, secs
            );
            ctx.reply(response).await?;
        } else {
            ctx.reply("Uptime tracking not initialized").await?;
        }
        Ok(())
    }
}
