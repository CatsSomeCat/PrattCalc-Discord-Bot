use log::error;
use serenity::all::*;
use serenity::builder::CreateEmbed;

use crate::discord::UserSession;

/// Handles the `/vars` slash command to display current session variables.
/// 
/// Shows a table of defined variables with their values and special styling
/// for neat presentation.
pub async fn handle_vars(
    context: &Context,
    interaction: &CommandInteraction,
    session: &UserSession,
) {
    // Format variables into a neat table
    let mut vars_list = String::new();
    let mut has_vars = false;
    
    // Use IntoIterator to iterate through the SymbolTable
    for (name, value) in session.variables.clone() {
        has_vars = true;
        let is_const = session.variables.is_constant(&name);
        let var_type = if is_const { "const" } else { "let" };
        vars_list.push_str(&format!("**{}** {} = {}\n", var_type, name, value));
    }
    
    let vars = if !has_vars {
        "_No variables set. Use expressions with '=' to define variables._".to_string()
    } else {
        vars_list
    };

    // Create response embed with formatting
    let embed = CreateEmbed::new()
        .title("Your Variables")
        .description(vars)
        .colour(Colour::GOLD);

    // Send the formatted response
    if let Err(error) = interaction
        .create_response(
            &context.http,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .embed(embed)
            ),
        )
        .await
    {
        error!("Failed to send vars command response: {:?}", error);
    }
} 
