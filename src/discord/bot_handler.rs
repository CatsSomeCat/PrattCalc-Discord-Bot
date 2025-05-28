use log::{info, error};
use serenity::all::*;
use std::collections::HashMap;

use crate::utils::extract_code_from_message;
use crate::discord::models::{Bot, UserSession, CommandMetadata, CommandMetadataContainer};
use crate::discord::commands;

#[async_trait]
impl EventHandler for Bot {
    /// Handles all interactions, such as, slash commands, context commands and etc.
    async fn interaction_create(&self, context: Context, interaction: Interaction) {
        match interaction {
            // Both slash commands and context menu commands now come through as Interaction::Command
            Interaction::Command(interaction) => {
                let user_id = interaction.user.id.get();
                let mut state_guard = self.state.lock().await;
                
                // Create a session with predefined constants if it doesn't exist
                let session = state_guard.sessions
                    .entry(user_id)
                    .or_insert_with(UserSession::new);

                // Handle slash command
                match interaction.data.name.as_str() {
                    "execute" => commands::handle_execute(&context, &interaction, session).await,
                    "evaluate" => commands::handle_evaluate(&context, &interaction, session).await,
                    "vars" => commands::handle_vars(&context, &interaction, session).await,
                    "clear" => commands::handle_clear(&context, &interaction, session).await,
                    "statistics" => commands::handle_statistics(&context, &interaction).await,
                    "help" => commands::handle_help(&context, &interaction).await,
                    "Execute Code" => {
                        // Handle message context menu command
                        if let Some(message) = interaction.data.resolved.messages.values().next() {
                            // Extract code from code blocks
                            if let Some(code) = extract_code_from_message(&message.content) {
                                // Use the existing session for evaluation
                                commands::handle_execute_code(&context, &interaction, session, &code).await;
                            } else {
                                // No code block found
                                interaction.create_response(&context.http, CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new()
                                        .content("No code block found in the selected message.")
                                        .ephemeral(false)
                                )).await.ok();
                            }
                        }
                    }
                    _ => {}
                }
            }
            // Handle component interactions (dropdown selections, buttons)
            Interaction::Component(interaction) => {
                // Try to handle help command dropdown interactions
                if commands::help::handle_help_component_interaction(&context, &interaction).await {
                    return;
                }
                
                // Add other component handlers here if needed
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
                .description("Computes a mathematical expression")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "expression",
                        "The mathematical expression to evaluate",
                    )
                    .required(true),
                ),
            CreateCommand::new("execute")
                .description("Executes calculator code")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String, 
                        "code",
                        "The code to execute",
                    )
                    .required(true),
                ),
            CreateCommand::new("vars")
                .description("Shows your stored variables"),
            CreateCommand::new("clear")
                .description("Removes all your variables and history"),
            CreateCommand::new("statistics")
                .description("Shows detailed system statistics information"),
            CreateCommand::new("help")
                .description("Shows detailed help for the calculator")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "topic",
                        "Specific help topic"
                    )
                    .required(false)
                    .add_string_choice("Overview", "0")
                    .add_string_choice("Basics", "1")
                    .add_string_choice("Syntax", "2")
                    .add_string_choice("Variables", "3")
                    .add_string_choice("Control Flow", "4")
                    .add_string_choice("Functions", "5")
                ),
        ];
        
        // Initialize command metadata
        let command_metadata = initialize_command_metadata();
        context.data.write().await.insert::<CommandMetadataContainer>(command_metadata);

        // Register each slash command globally
        for command in slash_commands {
            if let Err(error) = Command::create_global_command(&context.http, command).await {
                error!("Failed to register slash command: {:?}", error);
            }
        }

        // Register a message context menu command
        let context_menu = CreateCommand::new("Execute Code")
            .kind(CommandType::Message);
        if let Err(error) = Command::create_global_command(&context.http, context_menu).await {
            error!("Failed to register context command: {:?}", error);
        }
    }
}

/// Initialize metadata for all available bot commands.
/// 
/// Creates a structure containing command information including usage examples,
/// callback signatures, and descriptions for use in help commands.
pub fn initialize_command_metadata() -> HashMap<String, CommandMetadata> {
    info!("Initializing command metadata");
    
    let mut commands = HashMap::new();
    
    // Add evaluate command metadata
    commands.insert(
        "evaluate".to_string(),
        CommandMetadata {
            name: "evaluate".to_string(),
            description: "Evaluates a mathematical expression".to_string(),
            usage: "/evaluate <expression>".to_string(),
            examples: vec![
                "/evaluate 2 + 2 * 3".to_string(),
                "/evaluate 6 * 2".to_string(),
            ],
            callback_signature: "handle_evaluate(context, interaction, session)".to_string(),
        }
    );
    
    // Add execute command metadata
    commands.insert(
        "execute".to_string(),
        CommandMetadata {
            name: "execute".to_string(),
            description: "Executes calculator code".to_string(),
            usage: "/execute <code>".to_string(),
            examples: vec![
                "/execute let x = 10; x * 2".to_string(),
                "/execute { let sum = 0; let i = 1; while i <= 10 { sum += i; i += 1 }; sum }".to_string(),
            ],
            callback_signature: "handle_execute(context, interaction, session)".to_string(),
        }
    );
    
    // Add execute code context menu metadata
    commands.insert(
        "execute_code".to_string(),
        CommandMetadata {
            name: "Execute Code".to_string(),
            description: "Executes code from a message".to_string(),
            usage: "Right-click on a message > Apps > Execute Code".to_string(),
            examples: vec![
                "Right-click on message containing `2 + 2` > Apps > Execute Code".to_string(),
            ],
            callback_signature: "handle_execute_code(context, interaction, session, code)".to_string(),
        }
    );
    
    // Add vars command metadata
    commands.insert(
        "vars".to_string(),
        CommandMetadata {
            name: "vars".to_string(),
            description: "Shows your stored variables".to_string(),
            usage: "/vars".to_string(),
            examples: vec![
                "/vars".to_string(),
            ],
            callback_signature: "handle_vars(context, interaction, session)".to_string(),
        }
    );
    
    // Add clear command metadata
    commands.insert(
        "clear".to_string(),
        CommandMetadata {
            name: "clear".to_string(),
            description: "Removes all your variables and history".to_string(),
            usage: "/clear".to_string(),
            examples: vec![
                "/clear".to_string(),
            ],
            callback_signature: "handle_clear(context, interaction, session)".to_string(),
        }
    );
    
    // Add statistics command metadata
    commands.insert(
        "statistics".to_string(),
        CommandMetadata {
            name: "statistics".to_string(),
            description: "Shows detailed system statistics".to_string(),
            usage: "/statistics".to_string(),
            examples: vec![
                "/statistics".to_string(),
            ],
            callback_signature: "handle_statistics(context, interaction)".to_string(),
        }
    );
    
    // Add help command metadata
    commands.insert(
        "help".to_string(),
        CommandMetadata {
            name: "help".to_string(),
            description: "Shows help for the calculator".to_string(),
            usage: "/help <topic>".to_string(),
            examples: vec![
                "/help".to_string(),
                "/help syntax".to_string(),
                "/help variables".to_string(),
            ],
            callback_signature: "handle_help(context, interaction)".to_string(),
        }
    );
    
    commands
}
