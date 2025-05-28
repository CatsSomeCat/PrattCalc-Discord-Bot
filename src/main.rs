// Standard library imports
use std::env;
use std::sync::Arc;

// Async and synchronization
use tokio::sync::Mutex;

// Serenity (Discord library) imports
use serenity::all::*;

// Logging utilities
#[allow(unused_imports)]
use log::{info, debug, warn, trace, error};

// Environment configuration
use dotenv::dotenv;

// Import from our library crate
use ppaaeedb::discord::{Bot, SharedState, ShardManagerContainer, HelpEmbedsContainer, CommandMetadataContainer};
use ppaaeedb::discord::commands::help::initialize_help_embeds;
use ppaaeedb::discord::bot_handler::initialize_command_metadata;
use ppaaeedb::logging;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("RUST_BACKTRACE", "0");

    // Load environment variables from a .env file, ignoring errors if the file is missing
    dotenv().ok();

    // Set RUST_LOG if not already set
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "error");
    }

    // Initialize enhanced logger with custom configuration
    logging::setup_logger();

    // Retrieve the Discord bot token from environment variables
    let token: String = std::env::var("DISCORD_TOKEN")
        .expect("Cannot start bot because authentication credentials not found");

    // Specify only the gateway intents required for slash commands
    let intents: GatewayIntents = GatewayIntents::GUILD_INTEGRATIONS | GatewayIntents::GUILDS;

    // Build the Discord client with the token, intents, and an event handler
    let mut client: Client = Client::builder(&token, intents)
        .event_handler(Bot {
            state: Arc::new(Mutex::new(SharedState::default())),
        })
        .await
        .expect("Error creating Discord client");

    // This is necessary to access shard-specific information (like latency) from commands
    // Even with a single shard (default), Serenity uses a shard manager internally
    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
        
        // Initialize and store command metadata
        let command_metadata = initialize_command_metadata();
        data.insert::<CommandMetadataContainer>(command_metadata.clone());
        
        // Initialize and store help embeds
        let help_embeds = initialize_help_embeds(Some(&command_metadata));
        data.insert::<HelpEmbedsContainer>(help_embeds);
        
        info!("Initialized help embeds");
    }

    info!("Starting bot client");
    // Start the client, which will connect to Discord and begin handling events
    // If the client fails to start or crashes, log the error
    if let Err(error) = client.start().await {
        error!("Client error: {:?}", error);
    }

    Ok(())
}
