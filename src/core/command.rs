use super::Context;
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;

/// Trait that all commands must implement
#[async_trait]
pub trait Command: Send + Sync {
    /// Command name (e.g., "ping", "help")
    fn name(&self) -> &str;

    /// Command description
    fn description(&self) -> &str;

    /// Execute the command
    async fn execute(&self, ctx: Context, args: Vec<String>) -> Result<()>;

    /// Optional: command aliases
    fn aliases(&self) -> Vec<&str> {
        vec![]
    }
}

/// Registry for all available commands
pub struct CommandRegistry {
    commands: HashMap<String, Arc<dyn Command>>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    /// Register a command
    pub fn register(&mut self, command: Arc<dyn Command>) {
        let name = command.name().to_string();
        self.commands.insert(name.clone(), command.clone());

        // Register aliases
        for alias in command.aliases() {
            self.commands.insert(alias.to_string(), command.clone());
        }
    }

    /// Get a command by name
    pub fn get(&self, name: &str) -> Option<Arc<dyn Command>> {
        self.commands.get(name).cloned()
    }

    /// Get all commands (unique, no aliases)
    pub fn all(&self) -> Vec<Arc<dyn Command>> {
        let mut seen = std::collections::HashSet::new();
        let mut commands = Vec::new();

        for cmd in self.commands.values() {
            let name = cmd.name();
            if !seen.contains(name) {
                seen.insert(name.to_string());
                commands.push(cmd.clone());
            }
        }

        commands
    }

    /// Parse and execute a command from a message
    pub async fn execute(&self, message_content: &str, ctx: Context) -> Result<bool> {
        // Simple parsing: first word is command, rest are args
        let parts: Vec<&str> = message_content.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Ok(false);
        }

        let command_name = parts[0];
        let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

        if let Some(command) = self.get(command_name) {
            command.execute(ctx, args).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}
