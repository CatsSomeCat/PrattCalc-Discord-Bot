use log::error;
use serenity::all::*;
use serenity::builder::CreateEmbed;

use crate::discord::UserSession;
use crate::core::SymbolTable;

/// Handles the `/clear` slash command to reset user session.
/// 
/// Removes all variables and expression history for the user's session.
pub async fn handle_clear(
    context: &Context,
    interaction: &CommandInteraction,
    session: &mut UserSession,
) {
    // Reset the session
    session.variables = SymbolTable::<f32>::new();
    session.history.clear();

    // Create response embed
    let embed = CreateEmbed::new()
        .title("Session Cleared")
        .description("Your variables and command history have been reset.")
        .colour(Colour::RED);

    // Send confirmation
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
        error!("Failed to respond to clear command: {:?}", error);
    }
} 
