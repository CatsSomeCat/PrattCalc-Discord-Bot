use log::{info, error};
use serenity::all::*;
use serenity::builder::{
    CreateEmbed, 
    CreateSelectMenu, 
    CreateSelectMenuOption, 
    CreateSelectMenuKind,
    CreateActionRow, 
    CreateInteractionResponse, 
    CreateInteractionResponseMessage
};
use std::collections::HashMap;

use crate::discord::models::{HelpEmbedsContainer, CommandMetadataContainer};

/// Handles the `/help` command with detailed information about calculator usage.
///
/// Provides comprehensive documentation on syntax, variables, functions and examples.
pub async fn handle_help(
    context: &Context,
    interaction: &CommandInteraction,
) {
    // Check if a specific topic was requested
    let topic = interaction
        .data
        .options
        .first()
        .and_then(|opt| opt.value.as_str())
        .unwrap_or("overview")
        .to_lowercase();

    // Get the pre-created embed data from TypeMap
    let data_read = context.data.read().await;
    
    // Check if we're looking for help on a specific command
    let command_metadata = data_read.get::<CommandMetadataContainer>();
    if let Some(metadata_map) = command_metadata {
        if let Some(cmd_metadata) = metadata_map.get(&topic) {
            // Create and send a command-specific help embed
            let embed = create_command_help_embed(cmd_metadata);
            
            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .embed(embed)
            );

            if let Err(error) = interaction.create_response(&context.http, response).await {
                error!("Failed to send help response: {:?}", error);
            }
            return;
        }
    }
    
    // If not a command, use the standard help topic embeds
    let help_embeds = data_read.get::<HelpEmbedsContainer>()
        .expect("Expected HelpEmbedsContainer in TypeMap");
    
    // Get the requested embed or fall back to overview if not found
    let embed = help_embeds.get(&topic)
        .unwrap_or_else(|| help_embeds.get("0").unwrap())
        .clone();

    // If showing the main overview, add a dropdown for commands
    if topic == "overview" || topic == "0" || topic == "main" {
        if let Some(metadata_map) = command_metadata {
            let command_dropdown = create_command_dropdown(metadata_map);
            
            let response = CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .embed(embed)
                    .components(vec![command_dropdown])
            );

            if let Err(error) = interaction.create_response(&context.http, response).await {
                error!("Failed to send help response: {:?}", error);
            }
            return;
        }
    }
    
    // For other topics or if no command metadata available
    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .embed(embed)
    );

    if let Err(error) = interaction.create_response(&context.http, response).await {
        error!("Failed to send help response: {:?}", error);
    }
}

/// Creates a dropdown menu component listing all available commands.
/// 
/// The dropdown allows users to select a command to get detailed help information.
fn create_command_dropdown(command_metadata: &HashMap<String, crate::discord::models::CommandMetadata>) -> CreateActionRow {
    // Create options for each command; filter out "Execute Code" context command
    let commands: Vec<&crate::discord::models::CommandMetadata> = command_metadata.values()
        .filter(|cmd| cmd.name != "Execute Code") // Filter out only the context menu command
        .collect();
    
    // Create options with indices as values
    let options: Vec<CreateSelectMenuOption> = commands.iter()
        .enumerate()
        .map(|(index, cmd)| {
            CreateSelectMenuOption::new(&cmd.name, index.to_string())
                .description(&cmd.description)
        })
        .collect();
    
    // Create the select menu with the options
    let select_menu = CreateSelectMenu::new(
        "help_command_select", 
        CreateSelectMenuKind::String { options }
    ).placeholder("Select a command for detailed help");
    
    // Add the select menu to an action row
    CreateActionRow::SelectMenu(select_menu)
}

/// Handles interactions with the help command's dropdown menu.
///
/// When a user selects a command from the dropdown, this displays detailed help
/// for that specific command by updating the original message.
pub async fn handle_help_component_interaction(
    context: &Context,
    interaction: &ComponentInteraction,
) -> bool {
    // Check if this is our help command select menu
    if interaction.data.custom_id == "help_command_select" {
        // Get the selected value from the interaction data
        let selected_index = match &interaction.data.kind {
            ComponentInteractionDataKind::StringSelect { values } => values.first(),
            _ => None,
        };
        
        if let Some(index_str) = selected_index {
            // Parse the index
            if let Ok(index) = index_str.parse::<usize>() {
                // Get the command metadata
                let data_read = context.data.read().await;
                let command_metadata = data_read.get::<CommandMetadataContainer>();
                
                if let Some(metadata_map) = command_metadata {
                    // Get commands filtered the same way as in create_command_dropdown
                    let commands: Vec<&crate::discord::models::CommandMetadata> = metadata_map.values()
                        .filter(|cmd| cmd.name != "Execute Code")
                        .collect();
                    
                    // Get the command at the selected index
                    if let Some(cmd_metadata) = commands.get(index) {
                        // Create the command help embed
                        let embed = create_command_help_embed(cmd_metadata);
                        
                        // Update the original message with the command help
                        let response = CreateInteractionResponse::UpdateMessage(
                            CreateInteractionResponseMessage::new()
                                .embed(embed)
                                .components(vec![create_command_dropdown(metadata_map)]) // Keep the dropdown
                        );
                        
                        if let Err(error) = interaction.create_response(&context.http, response).await {
                            error!("Failed to update help response: {:?}", error);
                        }
                        
                        return true;
                    }
                }
            }
        }
    }
    
    false
}

/// Create a help embed for a specific command showing usage and examples.
/// 
/// Builds a detailed help message that includes command description, proper usage syntax,
/// and a list of practical examples.
fn create_command_help_embed(metadata: &crate::discord::models::CommandMetadata) -> CreateEmbed {
    let mut embed = CreateEmbed::new()
        .title("Command Help")
        .description(&metadata.description)
        .field("Usage", format!("`{}`", metadata.usage), false)
        .colour(Colour::BLUE);

    // Add examples if available
    if !metadata.examples.is_empty() {
        let examples = format!("```rust\n{}\n```", metadata.examples.join("\n"));
        embed = embed.field("Examples", examples, false);
    }
    
    // Add callback signature if available
    if !metadata.callback_signature.is_empty() {
        embed = embed.field("Callback", format!("```rust\n{}\n```", metadata.callback_signature), false);
    }
    
    embed
}

/// Initialize and configure pre-cached help embeds for different topics.
/// 
/// Creates a collection of embeds for each help topic to avoid rebuilding them on every request.
///
/// This improves response time and reduces code duplication.
pub fn initialize_help_embeds(command_metadata: Option<&HashMap<String, crate::discord::models::CommandMetadata>>) -> HashMap<String, CreateEmbed> {
    info!("Initializing help embeds cache");
    
    let mut embeds = HashMap::new();
    
    // Add the main help embed
    if let Some(metadata) = command_metadata {
        let overview = create_overview_help(metadata);
        embeds.insert("0".to_string(), overview.clone());
    }

    // Add the basic usage help embed
    embeds.insert(
        "1".to_string(),
        CreateEmbed::new()
            .title("Calculator Basics")
            .description("The calculator supports standard arithmetic operations and follows order of operations.")
            .field(
                "Simple Calculations",
                "Examples of basic calculations:\n\
                ```rust\n2 + 2 * 3\n(10 - 3) / 2\n```",
                false
            )
            .field(
                "Assignment",
                "```\nlet x = 5       // Declare x with value 5\nx = 10          // Change x to 10\nx += 2          // Add 2 to x (now 12)\n```",
                false
            )
            .field(
                "Constants",
                "```\nconst PI = 3.14159\n// Constants cannot be changed\n```",
                false
            )
            .colour(Colour::from_rgb(50, 168, 82))
    );
    
    // Add the syntax help embed
    embeds.insert(
        "2".to_string(),
        create_syntax_help()
    );
    
    // Add the variables help embed
    embeds.insert(
        "3".to_string(),
        create_variables_help()
    );
    
    // Add the control flow help embed
    embeds.insert(
        "4".to_string(),
        CreateEmbed::new()
            .title("Control Flow")
            .description("Control flow statements let you make decisions and repeat calculations.")
            .field(
                "If Statements",
                "```rust\nif x > 5 {\n  x = x * 2\n} else {\n  x = x + 1\n}\n```",
                false
            )
            .field(
                "While Loops",
                "```rust\nlet i = 1\nlet factorial = 1\nwhile i <= 5 {\n  factorial *= i\n  i += 1\n}\n// factorial now equals 120\n```",
                false
            )
            .field(
                "Block Statements",
                "```rust\n// Blocks create temporary scopes\n{\n  let temp = x * 2\n  y = temp + 1\n}\n// temp is no longer accessible\n```",
                false
            )
            .colour(Colour::from_rgb(194, 124, 14))
    );
    
    // Add the functions help embed
    embeds.insert(
        "5".to_string(),
        create_functions_help()
    );
    
    embeds
}

/// Creates the overview help embed with general information.
fn create_overview_help(command_metadata: &HashMap<String, crate::discord::models::CommandMetadata>) -> CreateEmbed {
    // Generate command list from metadata; filter out "Execute Code" context command
    let commands_list = command_metadata.values()
        .filter(|cmd| cmd.name != "Execute Code") // Filter out the context menu command
        .map(|cmd| format!("`/{}` - {}", cmd.name, cmd.description))
        .collect::<Vec<_>>()
        .join("\n");
    
    CreateEmbed::new()
        .title("Calculator Help")
        .description("This calculator bot allows you to evaluate mathematical expressions, store variables, use control flow structures, and define custom functions and procedures.")
        .field("Available Commands", commands_list, false)
        .field("Help Topics", 
               "`basics` - Basic usage and expressions\n\
                `syntax` - Expression syntax and operators\n\
                `variables` - Working with variables\n\
                `control` - Control flow structures\n\
                `functions & procedures` - Built-in and user-defined functions/procedures", 
                false)
        .field("Examples", 
               "```rust\n2 + 2 * 3;\n(10 - 5) / 2;\nlet x = 5;\n\n// Define a function\nfn square(x) {\n    return x * x\n}\n```", 
                false)
        .colour(Colour::BLUE)
}

/// Creates the basics help embed with fundamental information.
#[allow(dead_code)]
fn create_basics_help() -> CreateEmbed {
    CreateEmbed::new()
        .title("Basic Calculator Usage")
        .description("Learn the fundamentals of using the calculator bot.")
        .field("Simple Expressions", 
               "Examples of basic calculations:\n\
                ```rust\n2 + 2 * 3;\n(10 - 3) / 2;\n5 ^ 2;\n16 % 3;\n```", 
               false)
        .field("Order of Operations", 
               "The calculator follows standard PEMDAS:\n\
                ```\nParentheses: ()\nExponents: ^\nMultiplication & Division: * /\nAddition & Subtraction: + -\n```", 
               false)
        .field("Session Management", 
               "Your calculations persist across interactions.\n\
                Clear your session with `/clear`.", 
               false)
        .colour(Colour::from_rgb(65, 105, 225))
}

/// Creates the syntax help embed with detailed operator information.
fn create_syntax_help() -> CreateEmbed {
    CreateEmbed::new()
        .title("Syntax Help")
        .description("Learn about the basic syntax elements and operators of the calculator.")
        .field("Literals", 
               "```\nNumbers: 123, 3.14, 0xFF (hex), 0b1010 (binary)\nVariables: x, counter, result\nKeywords: true (1), false (0)\n```", 
               false)
        .field("Arithmetic", 
               "```\nAddition: a + b\nSubtraction: a - b\nMultiplication: a * b\nDivision: a / b\nModulo: a % b\nPower: a ^ b\nRoot: b √ a (b'th root of a)\n```", 
               false)
        .field("Comparison", 
               "```\nEqual: a == b\nNot equal: a != b\nGreater: a > b\nLess: a < b\nGreater or equal: a >= b\nLess or equal: a <= b\n```", 
               false)
        .field("Logical", 
               "```\nAND: a && b (1 if both a and b are non-zero)\nOR: a || b (1 if either a or b is non-zero)\nXOR: a ^^ b (1 if exactly one of a or b is non-zero)\nNOT: !a (1 if a is zero, 0 otherwise)\n```", 
               false)
        .field("Advanced Logical", 
               "```\nNAND: a !& b (NOT of AND)\nNOR: a !| b (NOT of OR)\nXNOR: a !^ b (NOT of XOR)\n```", 
               false)
        .colour(Colour::from_rgb(100, 149, 237))
}

/// Creates the variables help embed with variable usage information.
fn create_variables_help() -> CreateEmbed {
    CreateEmbed::new()
        .title("Working with Variables")
        .description("Variables let you store values for later use.")
        .field("Declaring Variables", 
               "Use `let` to declare variables:\n\
                ```rust\nlet x = 42;\nlet result = x * 2;\n```", 
               false)
        .field("Constants", 
               "Use `const` for immutable values:\n\
                ```rust\nconst MY_CONSTANT = 3.14159;\nconst GRAVITY = 9.81;\n```\n\
                Constants cannot be modified after declaration.", 
               false)
        .field("Predefined Constants", 
               "The calculator comes with built-in mathematical constants:\n\
                ```\n• π (3.14159...)\n• τ (2π, 6.28318...)\n• Euler's number (2.71828...)\n• Golden ratio (1.61803...)\n• Square root of 2 (1.41421...)\n• Positive infinity\n```\n\
                Access these via their reserved names (PI, TAU, E, PHI, SQRT2, INFINITY).", 
               false)
        .field("Assignment", 
               "Update existing variables:\n\
                ```rust\nx = x + 1;\nx += 5;\ny *= 2;\n```\n\
                Note: Variables must be declared with `let` first.", 
               false)
        .colour(Colour::from_rgb(70, 130, 180))
}

/// Creates the control flow help embed with information about conditionals and loops.
#[allow(dead_code)]
fn create_control_flow_help() -> CreateEmbed {
    CreateEmbed::new()
        .title("Control Flow")
        .description("Control flow structures allow for complex calculations.")
        .field("If-Else Statements", 
               "Conditional execution:\n\
                ```rust\nif x > 10 { let result = x * 2; } else { let result = x / 2; }\n```\n\
                You can also use `else if`:\n\
                ```rust\nif x < 0 { -x; } else if x > 100 { 100; } else { x; }\n```", 
               false)
        .field("While Loops", 
               "Repeat expressions while a condition is true:\n\
                ```rust\nlet i = 0; let sum = 0; while i < 10 { sum += i; i += 1; }\n```", 
               false)
        .field("Break and Continue", 
               "Control loop execution:\n\
                ```rust\nwhile x < 100 { x += 10; if x > 50 { break; } }\n```\n\
                ```rust\nwhile x < 10 { x += 1; if x % 2 == 0 { continue; } }\n```", 
               false)
        .field("Code Blocks", 
               "Group multiple statements with curly braces `{ }`\n\
                Separate statements with semicolons `;`", 
               false)
        .colour(Colour::from_rgb(75, 0, 130))
}

/// Creates the functions help embed with information about built-in functions.
fn create_functions_help() -> CreateEmbed {
    CreateEmbed::new()
        .title("Calculator Functions")
        .description("The calculator supports built-in mathematical functions and user-defined functions & procedures.")
        .field("Basic Trigonometric", 
               "```rust\nsin(x) - Sine of x (radians)\ncos(x) - Cosine of x (radians)\ntan(x) - Tangent of x (radians)\n```", 
               false)
        .field("Additional Trigonometric", 
               "```rust\ncot(x) - Cotangent of x (radians)\nsec(x) - Secant of x (radians)\ncsc(x) - Cosecant of x (radians)\n```", 
               false)
        .field("Inverse Trigonometric", 
               "```rust\nasin(x) - Arc sine (inverse sine)\nacos(x) - Arc cosine (inverse cosine)\natan(x) - Arc tangent (inverse tangent)\natan2(y, x) - Arc tangent of y/x with quadrant\n```", 
               false)
        .field("Math Functions", 
               "```rust\nlog(x) - Natural logarithm of x\nsqrt(x) - Square root of x\nabs(x) - Absolute value of x\n```", 
               false)
        .field("Min/Max Functions", 
               "```rust\nmin(x, y) - Minimum of x and y\nmax(x, y) - Maximum of x and y\n```", 
               false)
        .field("Random Number Generator", 
               "```rust\nrand() - Random number between 0 and 1\nrand(max) - Random number between 0 and max\nrand(min, max) - Random number between min and max\n```", 
               false)
        .field("Function Usage", 
               "```rust\nsin(PI / 2);\natan2(1, -1);\nsqrt(25) + abs(-10);\n```", 
               false)
        .field("User-Defined Functions",
               "Define your own reusable functions that return values:\n```rust\nfn square(x) {\n    return x * x\n}\n\nfn hypotenuse(a, b) {\n    return sqrt(a * a + b * b)\n}\n\n// Use your functions\nlet area = square(5);  // 25\nlet c = hypotenuse(3, 4);  // 5\n```",
               false)
        .field("User-Defined Procedures",
               "Define procedures that perform operations without returning values:\n```rust\nlet total = 0;\n\nproc add_values(a, b) {\n    total = total + a + b;\n}\n\n// Use your procedure\nadd_values(5, 10);  // total is now 15\n```",
               false)
        .field("The End Keyword",
               "Terminate program execution and return a value:\n```rust\nlet x = calculate_value();\nif x > threshold {\n    end x;  // Exits with value x\n}\n// Code continues if x <= threshold\n```",
               false)
        .colour(Colour::from_rgb(138, 43, 226))
} 
