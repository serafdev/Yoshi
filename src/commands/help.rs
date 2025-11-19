use crate::core::{Command, Context};
use anyhow::Result;
use async_trait::async_trait;

pub struct HelpCommand;

#[async_trait]
impl Command for HelpCommand {
    fn name(&self) -> &str {
        "help"
    }

    fn description(&self) -> &str {
        "Show available commands"
    }

    async fn execute(&self, ctx: Context, _args: Vec<String>) -> Result<()> {
        let help_text = r#"
🤖 **Yoshi Bot - Available Commands**

**Basic Commands:**
• `ping` - Check if the bot is alive
• `help` - Show this help message
• `status` - Show infrastructure status

**Infrastructure Commands:**
• `status` - Check system status
• More commands coming soon!

Just add a new file in `src/commands/` to add a new command!
"#;
        ctx.reply(help_text).await?;
        Ok(())
    }

    fn aliases(&self) -> Vec<&str> {
        vec!["h", "commands"]
    }
}
