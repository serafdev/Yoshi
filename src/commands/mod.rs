// Commands are auto-discovered from this module
// Each command file should implement the Command trait
// Simply add a new .rs file and register it here!

pub mod ping;
pub mod help;
pub mod status;
pub mod uptime;
pub mod system;
pub mod version;

use crate::core::{Command, CommandRegistry};
use std::sync::Arc;

/// Auto-register all commands
pub fn register_all(registry: &mut CommandRegistry) {
    // Initialize uptime tracking
    uptime::UptimeCommand::init();

    // Register all commands
    registry.register(Arc::new(ping::PingCommand));
    registry.register(Arc::new(help::HelpCommand));
    registry.register(Arc::new(status::StatusCommand));
    registry.register(Arc::new(uptime::UptimeCommand));
    registry.register(Arc::new(system::SystemCommand));
    registry.register(Arc::new(version::VersionCommand));
}
