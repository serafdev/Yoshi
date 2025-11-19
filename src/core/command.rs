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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Message, MessageContent, Author, Responder};
    use std::sync::Mutex;

    struct TestCommand {
        name: String,
        description: String,
        executions: Arc<Mutex<Vec<Vec<String>>>>,
    }

    #[async_trait]
    impl Command for TestCommand {
        fn name(&self) -> &str {
            &self.name
        }

        fn description(&self) -> &str {
            &self.description
        }

        async fn execute(&self, _ctx: Context, args: Vec<String>) -> Result<()> {
            self.executions.lock().unwrap().push(args);
            Ok(())
        }

        fn aliases(&self) -> Vec<&str> {
            if self.name == "test" {
                vec!["t"]
            } else {
                vec![]
            }
        }
    }

    struct TestResponder {
        messages: Arc<Mutex<Vec<String>>>,
    }

    #[async_trait]
    impl Responder for TestResponder {
        async fn send_message(&self, _channel_id: &str, content: String) -> Result<()> {
            self.messages.lock().unwrap().push(content);
            Ok(())
        }
    }

    fn create_test_context() -> Context {
        let message = Message {
            id: "1".to_string(),
            author: Author {
                id: "user1".to_string(),
                name: "Test User".to_string(),
                is_bot: false,
            },
            content: MessageContent::Text("test message".to_string()),
            channel_id: "channel1".to_string(),
            platform: "test".to_string(),
        };

        let responder = Arc::new(TestResponder {
            messages: Arc::new(Mutex::new(Vec::new())),
        });

        Context::new(message, responder)
    }

    #[test]
    fn test_registry_new() {
        let registry = CommandRegistry::new();
        assert_eq!(registry.all().len(), 0);
    }

    #[test]
    fn test_registry_register() {
        let mut registry = CommandRegistry::new();
        let executions = Arc::new(Mutex::new(Vec::new()));

        let cmd = Arc::new(TestCommand {
            name: "test".to_string(),
            description: "A test command".to_string(),
            executions: executions.clone(),
        });

        registry.register(cmd);

        assert_eq!(registry.all().len(), 1);
        assert!(registry.get("test").is_some());
        assert!(registry.get("t").is_some()); // alias
    }

    #[tokio::test]
    async fn test_registry_execute() {
        let mut registry = CommandRegistry::new();
        let executions = Arc::new(Mutex::new(Vec::new()));

        let cmd = Arc::new(TestCommand {
            name: "test".to_string(),
            description: "A test command".to_string(),
            executions: executions.clone(),
        });

        registry.register(cmd);

        let ctx = create_test_context();
        let result = registry.execute("test arg1 arg2", ctx).await.unwrap();

        assert!(result);
        let execs = executions.lock().unwrap();
        assert_eq!(execs.len(), 1);
        assert_eq!(execs[0], vec!["arg1", "arg2"]);
    }

    #[tokio::test]
    async fn test_registry_execute_alias() {
        let mut registry = CommandRegistry::new();
        let executions = Arc::new(Mutex::new(Vec::new()));

        let cmd = Arc::new(TestCommand {
            name: "test".to_string(),
            description: "A test command".to_string(),
            executions: executions.clone(),
        });

        registry.register(cmd);

        let ctx = create_test_context();
        let result = registry.execute("t", ctx).await.unwrap();

        assert!(result);
        assert_eq!(executions.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_registry_execute_not_found() {
        let registry = CommandRegistry::new();
        let ctx = create_test_context();
        let result = registry.execute("notfound", ctx).await.unwrap();

        assert!(!result);
    }

    #[tokio::test]
    async fn test_registry_execute_empty() {
        let registry = CommandRegistry::new();
        let ctx = create_test_context();
        let result = registry.execute("", ctx).await.unwrap();

        assert!(!result);
    }
}
