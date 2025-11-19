use crate::core::{Command, Context};
use anyhow::Result;
use async_trait::async_trait;

pub struct StatusCommand;

#[async_trait]
impl Command for StatusCommand {
    fn name(&self) -> &str {
        "status"
    }

    fn description(&self) -> &str {
        "Check infrastructure status"
    }

    async fn execute(&self, ctx: Context, args: Vec<String>) -> Result<()> {
        let target = args.first().map(|s| s.as_str()).unwrap_or("all");

        let status = match target {
            "all" => {
                format!(
                    "📊 **Infrastructure Status**\n\n\
                     ✅ Bot: Online\n\
                     ✅ Platform: {}\n\
                     ✅ Uptime: Running\n\
                     🔧 Commands: Loaded\n",
                    ctx.message.platform
                )
            }
            "bot" => "✅ Bot is running normally".to_string(),
            service => format!("Status for '{}': Not implemented yet. Add your infrastructure checks here!", service),
        };

        ctx.reply(status).await?;
        Ok(())
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["health", "check"]
    }
}
