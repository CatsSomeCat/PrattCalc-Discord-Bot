use log::error;
use serenity::all::*;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};

use crate::discord::error_handler::send_error;
use crate::discord::UserSession;

/// Handles the `/evaluate` slash command for mathematical expressions.
/// 
/// Supports variable assignments, control flow structures, and complex calculations
/// with detailed error reporting.
pub async fn handle_evaluate(
    context: &Context,
    interaction: &CommandInteraction,
    session: &mut UserSession,
) {
    // Extract and clean input
    let input = interaction
        .data
        .options
        .first()
        .and_then(|opt| opt.value.as_str())
        .unwrap_or("")
        .trim();

    if input.is_empty() {
        send_error(context, interaction, "Please provide an expression to evaluate.", None).await;
        return;
    }

    // Use the evaluate function from core to evaluate the input
    let result = match crate::core::evaluate(input, &mut session.variables) {
        Ok(value) => value,
        Err(error) => {
            // Format error messages differently based on type
            let error_message = match &error {
                &crate::core::CalcError::Parse(ref parse_err) => {
                    format!("```fix\n{}\n```", parse_err)
                },
                &crate::core::CalcError::Eval(ref eval_err) => {
                    format!("```fix\n{}\n```", eval_err)
                },
                &crate::core::CalcError::Exec(ref exec_err) => {
                    format!("```fix\n{}\n```", exec_err)
                },
            };
            
            send_error(context, interaction, &error_message, None).await;
            return;
        }
    };
    
    // Save to history
    session.history.push(input.to_string());
    
    // Create description with the result (evaluate always returns a value)
    let description = format!(
        "**Code:**\n```rs\n{}\n```\n\
        **Result:**\n```rs\n{}\n```",
        input.trim(),
        result
    );

    // Create response embed
    let embed = CreateEmbed::new()
        .title("Expression Evaluation Successful")
        .description(description)
        .colour(Colour::DARK_GREEN)
        .footer(CreateEmbedFooter::new(format!(
            "Session contains {} variables and {} history entries!",
            session.variables.len(),
            session.history.len()
        )));

    let response = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new()
            .embed(embed)
    );

    if let Err(error) = interaction.create_response(&context.http, response).await {
        error!("Failed to respond to evaluate command: {:?}", error);
    }
} 
