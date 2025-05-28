// The main library file that exports all modules

// Core components
#[path = "core/lib.rs"]
pub mod core;

// Discord bot components
#[path = "discord/lib.rs"]
pub mod discord;

// Utils
pub mod utils;

// Logging
pub mod logging; 
