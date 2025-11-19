use crate::core::{Command, Context};
use anyhow::Result;
use async_trait::async_trait;

pub struct VersionCommand;

#[async_trait]
impl Command for VersionCommand {
    fn name(&self) -> &str {
        "version"
    }

    fn description(&self) -> &str {
        "Show bot version"
    }

    async fn execute(&self, ctx: Context, _args: Vec<String>) -> Result<()> {
        let version = env!("CARGO_PKG_VERSION");
        let name = env!("CARGO_PKG_NAME");

        ctx.reply(format!("🤖 **{}** v{}", name, version)).await?;
        Ok(())
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["v", "ver"]
    }
}
