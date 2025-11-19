// Commands are auto-discovered from this module
// Each command file should implement the Command trait

pub mod ping;
pub mod help;
pub mod status;

use crate::core::{Command, CommandRegistry};
use std::sync::Arc;

/// Auto-register all commands
pub fn register_all(registry: &mut CommandRegistry) {
    registry.register(Arc::new(ping::PingCommand));
    registry.register(Arc::new(help::HelpCommand));
    registry.register(Arc::new(status::StatusCommand));
}
