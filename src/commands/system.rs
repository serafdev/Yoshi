use crate::core::{Command, Context};
use anyhow::Result;
use async_trait::async_trait;

pub struct SystemCommand;

#[async_trait]
impl Command for SystemCommand {
    fn name(&self) -> &str {
        "system"
    }

    fn description(&self) -> &str {
        "Show system information"
    }

    async fn execute(&self, ctx: Context, _args: Vec<String>) -> Result<()> {
        let os = std::env::consts::OS;
        let arch = std::env::consts::ARCH;
        let version = env!("CARGO_PKG_VERSION");

        let response = format!(
            "🖥️ **System Information**\n\n\
             • OS: {}\n\
             • Architecture: {}\n\
             • Bot Version: v{}\n\
             • Platform: {}\n\
             • Rust: {}",
            os,
            arch,
            version,
            ctx.message.platform,
            rustc_version_runtime::version()
        );

        ctx.reply(response).await?;
        Ok(())
    }
}
