// Discord client module for handling bot interactions

pub mod commands;
mod error_handler;
mod models;
pub mod bot_handler;

// Re-export for easier access
pub use models::{Bot, UserSession, SharedState, ShardManagerContainer, HelpEmbedsContainer, CommandMetadataContainer};
pub use error_handler::send_error; 