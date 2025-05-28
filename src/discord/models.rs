use std::collections::HashMap;
use std::sync::Arc;

use serenity::all::*;
use serenity::gateway::ShardManager;
use serenity::prelude::*;
use tokio::sync::Mutex;

use crate::core::SymbolTable;

/// This implementation tells the TypeMap that `ShardManagerContainer` is the key, and its
/// associated value is an `Arc<ShardManager>` object.
pub struct ShardManagerContainer;

// A TypeMap key used to store and access the ShardManager instance in Serenity's Context.
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<ShardManager>;
}

/// Stores pre-created help embeds to avoid creating them on each command use.
pub struct HelpEmbedsContainer;

impl TypeMapKey for HelpEmbedsContainer {
    type Value = HashMap<String, CreateEmbed>;
}

/// Stores metadata about available commands for help and documentation.
pub struct CommandMetadataContainer;

impl TypeMapKey for CommandMetadataContainer {
    type Value = HashMap<String, CommandMetadata>;
}

/// Metadata for a single command including its usage and examples.
#[derive(Clone)]
pub struct CommandMetadata {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub examples: Vec<String>,
    pub callback_signature: String,
}

/// Holds each user's variables and input history.
#[derive(Default)]
pub struct UserSession {
    pub variables: SymbolTable<f32>,
    pub history: Vec<String>,
}

impl UserSession {
    /// Creates a new user session with predefined mathematical constants.
    pub fn new() -> Self {
        Self {
            variables: SymbolTable::new(),
            history: Vec::new(),
        }
    }
}

/// Entire bot state shared across users.
#[derive(Default)]
pub struct SharedState {
    pub sessions: HashMap<u64, UserSession>,
}

/// Main bot structure with shared state.
pub struct Bot {
    pub state: Arc<Mutex<SharedState>>,
} 
