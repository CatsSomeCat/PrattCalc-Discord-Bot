// Standard library imports
use std::collections::HashMap;
use std::sync::Arc;

#[allow(unused_imports)]
use std::time::{Instant, Duration};

// Async and synchronization
use tokio::sync::Mutex;

// Serenity (Discord library) imports
use serenity::all::*;
// use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
// use serenity::model::id::GuildId;
// use serenity::prelude::*;

// System monitoring
use sysinfo::{
    System, 
    SystemExt, 
    ComponentExt, 
    CpuExt
};

// Logging utilities
#[allow(unused_imports)]
use log::{info, error, warn, debug};

// Environment configuration
use dotenv::dotenv;

// Only compile during tests
#[cfg(test)] 
mod tests;

// Local modules
mod parser;

// Re-exports from local modules
#[allow(unused_imports)]
use parser::{Expression, ParseError};

/// Holds each user's variables and input history.
#[derive(Default)]
struct UserSession {
    variables: HashMap<String, f32>,
    history: Vec<String>,
}

/// Entire bot state shared across users.
#[derive(Default)]
struct SharedState {
    sessions: HashMap<u64, UserSession>,
}

/// Main bot structure with shared state.
struct Bot {
    state: Arc<Mutex<SharedState>>,
}

#[async_trait]
impl EventHandler for Bot {
    /// Handles all interactions: slash commands and context commands.
    async fn interaction_create(&self, context: Context, interaction: Interaction) {
        match interaction {
            // Both slash commands and context menu commands now come through as Interaction::Command
            Interaction::Command(command_interaction) => {
                let user_id = command_interaction.user.id.get();
                let mut state_guard = self.state.lock().await;
                let session = state_guard.sessions.entry(user_id).or_default();

                // Otherwise handle as slash command
                match command_interaction.data.name.as_str() {
                    "evaluate" => handle_evaluate(&context, &command_interaction, session).await,
                    "vars" => handle_vars(&context, &command_interaction, session).await,
                    "clear" => handle_clear(&context, &command_interaction, session).await,
                    "status" => handle_status(&context, &command_interaction).await,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    /// Once connected, register commands with Discord.
    async fn ready(&self, context: Context, ready: Ready) {
        info!("Logged in as {} ({})", ready.user.name, ready.user.id);

        // Define slash commands
        let slash_commands = vec![
            CreateCommand::new("evaluate")
                .description("Computes an arithmetic expression")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "expression",
                        "The expression to calculate",
                    )
                    .required(true),
                ),
            CreateCommand::new("vars")
                .description("Shows your stored variables"),
            CreateCommand::new("clear")
                .description("Removes all your variables and history"),
            CreateCommand::new("status")
                .description("Shows detailed system status information"),
        ];

        // Register each slash command globally
        for command in slash_commands {
            if let Err(e) = Command::create_global_command(&context.http, command).await {
                error!("Failed to register slash command: {:?}", e);
            }
        }

        // Register a message context menu command
        let context_menu = CreateCommand::new("Evaluate code block")
            .kind(CommandType::Message);
        if let Err(e) = Command::create_global_command(&context.http, context_menu).await {
            error!("Failed to register context command: {:?}", e);
        }
    }
}

/// Handles the `/evaluate` slash command for mathematical expressions.
/// Supports variable assignments and complex calculations with clear error reporting.
async fn handle_evaluate(
    http_context: &Context,
    command_interaction: &CommandInteraction,
    session: &mut UserSession,
) {
    // Extract and clean input
    let input = command_interaction
        .data
        .options
        .first()
        .and_then(|opt| opt.value.as_str())
        .unwrap_or("")
        .trim();

    if input.is_empty() {
        send_error(http_context, command_interaction, "Please provide an expression to evaluate.").await;
        return;
    }

    // Parse expressions with detailed error handling
    let expressions = match Expression::parse_from_str(input) {
        Ok(list) if !list.is_empty() => list,
        Ok(_) => {
            send_error(
                http_context,
                command_interaction,
                "No valid expressions found in input.",
            ).await;
            return;
        }
        Err(e) => {
            send_error(
                http_context,
                command_interaction,
                &format!("```fix\nSyntax Error:\n{}\n```", e),
            ).await;
            return;
        }
    };

    // Evaluate expressions with progress tracking
    let mut last_value = 0.0;
    for (i, expr) in expressions.iter().enumerate() {
        let result = if let Some((var_name, rhs_expr)) = expr.is_assignment() {
            // Variable assignment with limit enforcement
            if !session.variables.contains_key(&var_name) && session.variables.len() >= 25 {
                send_error(
                    http_context,
                    command_interaction,
                    "Variable limit reached (max 25).\nUse `/clear` to reset your session.",
                ).await;
                return;
            }

            match rhs_expr.evaluate(&session.variables) {
                Ok(val) => {
                    session.variables.insert(var_name.clone(), val);
                    Some((format!("{} = {}", var_name, val), val))
                }
                Err(err) => {
                    send_error(
                        http_context,
                        command_interaction,
                        &format!("```fix\nError in expression {}:\n{}\n```", i + 1, err),
                    ).await;
                    return;
                }
            }
        } else {
            match expr.evaluate(&session.variables) {
                Ok(val) => Some((format!("[{}] = {}", i + 1, val), val)),
                Err(err) => {
                    send_error(
                        http_context,
                        command_interaction,
                        &format!("```fix\nError in expression {}:\n{}\n```", i + 1, err),
                    ).await;
                    return;
                }
            }
        };

        if let Some((_step_result, val)) = result {
            last_value = val;
        }
    }

    // Save to history and send success response
    session.history.push(input.to_string());
    let embed = CreateEmbed::new()
        .title("Evaluation Successful")
        .description(format!("```\n{}\n```\n**Result:** `{}`", input, last_value))
        .colour(Colour::DARK_GREEN)
        .footer(CreateEmbedFooter::new(format!(
            "Session contains {} variables | History length: {}",
            session.variables.len(),
            session.history.len()
        )));

    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .embed(embed)
            .ephemeral(false),
    );

    if let Err(err) = command_interaction.create_response(&http_context.http, response).await {
        error!("Failed to send evaluation response: {:?}", err);
    }
}

/// Handles the `/vars` command to display all user variables in a formatted list.
async fn handle_vars(
    http_context: &Context,
    command_interaction: &CommandInteraction,
    session: &UserSession,
) {
    let description = if session.variables.is_empty() {
        "No variables stored in this session.".to_string()
    } else {
        format!(
            "```prolog\n{}\n```\n**Total:** {} variables",
            session
                .variables
                .iter()
                .map(|(k, v)| format!("{:>10} = {:<12.6}", k, v))
                .collect::<Vec<_>>()
                .join("\n"),
            session.variables.len()
        )
    };

    let embed = CreateEmbed::new()
        .title("📊 Your Variables")
        .description(description)
        .colour(Colour::BLITZ_BLUE)
        .footer(CreateEmbedFooter::new(
            "Use `/evaluate` to modify or `/clear` to reset",
        ));

    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .embed(embed)
            .ephemeral(false),
    );

    if let Err(err) = command_interaction.create_response(&http_context.http, response).await {
        error!("Failed to send variables response: {:?}", err);
    }
}

/// Handles the /clear slash command.
async fn handle_clear(
    http_context: &Context,
    command_interaction: &CommandInteraction,
    session: &mut UserSession,
) {
    session.variables.clear();
    session.history.clear();
    let embed = CreateEmbed::new()
        .title("Cleared All Data")
        .description("All your variables and history have been reset.")
        .colour(Colour::GOLD);

    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().embed(embed),
    );
    if let Err(e) = command_interaction.create_response(&http_context.http, response).await {
        error!("Failed to send clear response: {:?}", e);
    }
}

// Helper functions for formatting system info
fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    format!("`{}d {}h {}m`", days, hours, minutes)
}

fn format_memory(system: &System) -> String {
    let used_mem = system.used_memory() as f64 / 1024.0 / 1024.0;
    let total_mem = system.total_memory() as f64 / 1024.0 / 1024.0;
    format!("`{:.1}MB / {:.1}MB ({:.0}%)`", 
           used_mem, 
           total_mem,
           (used_mem / total_mem) * 100.0)
}

fn format_cpu(system: &System) -> String {
    let global_cpu = system.global_cpu_info();
    format!("`{:.1}%` usage ({} cores)", 
           global_cpu.cpu_usage(),
           system.cpus().len())
}

fn format_temperature(system: &System) -> String {
    system.components()
        .iter()
        .find(|c| c.label().contains("CPU"))
        .map_or("`N/A`".to_string(), |c| {
            format!("`{:.1}°C`", c.temperature())
        })
}

/// Handles the /status slash command with detailed system info
async fn handle_status(
    http_context: &Context,
    command_interaction: &CommandInteraction,
) {
    // Start timing for latency measurement
    let start_time = Instant::now();

    // Get system information
    let mut system = System::new_all();
    system.refresh_all();

    // Calculate latency
    let latency = start_time.elapsed();

    // Format system information
    let embed = CreateEmbed::new()
        .title("System Status")
        .colour(Colour::DARK_GREEN)
        .field("Bot Latency", format!("`{}ms`", latency.as_millis()), true)
        .field("Uptime", format_uptime(system.uptime()), true)
        .field("Memory", format_memory(&system), false)
        .field("CPU", format_cpu(&system), false)
        .field("Processes", format!("`{}` running", system.processes().len()), true)
        .field("Temperature", format_temperature(&system), true);

    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().embed(embed)
    );

    if let Err(err) = command_interaction.create_response(&http_context.http, response).await {
        error!("Failed to send status response: {:?}", err);
    }
}

/// Utility: send a simple error embed with a formatted message.
/// The error message will be displayed in a clean code block format.
async fn send_error(
    context: &Context,
    interaction: &CommandInteraction,
    message: &str,
) {
    let embed = CreateEmbed::new()
        .title("Error")
        .description(message)
        .colour(Colour::DARK_RED);
    
    if let Err(err) = interaction.create_response(
        &context.http,
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().embed(embed)
        )
    ).await {
        error!("Failed to send error message: {}", err);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok(); env_logger::init();

    let token = std::env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in .env");
    let intents = GatewayIntents::non_privileged();

    let mut client = Client::builder(&token, intents)
        .event_handler(Bot { state: Arc::new(Mutex::new(SharedState::default())) })
        .await
        .expect("Error creating client");

    if let Err(err) = client.start().await {
        error!("Client error: {:?}", err);
    }
}