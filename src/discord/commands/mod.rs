// Commands module for Discord bot

mod evaluate;
mod executor;
mod vars;
mod clear;
mod statistics;
pub mod help;

// Re-export command handlers
pub use evaluate::handle_evaluate;
pub use executor::handle_execute;
pub use executor::handle_execute_code;
pub use vars::handle_vars;
pub use clear::handle_clear;
pub use statistics::handle_statistics;
pub use help::handle_help;
pub use help::handle_help_component_interaction; 
