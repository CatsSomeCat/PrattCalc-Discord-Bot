use log::error;
use serenity::all::*;
use serenity::builder::{CreateEmbed, CreateEmbedFooter};

use crate::discord::error_handler::send_error;
use crate::discord::UserSession;

/// Handles the `/execute` slash command for executing calculator code.
/// 
/// Similar to evaluate but emphasizes code execution with support for
/// multiline code blocks and complex logic.
pub async fn handle_execute(
    context: &Context,
    interaction: &CommandInteraction,
    session: &mut UserSession,
) {
    // Extract and clean input
    let code = interaction
        .data
        .options
        .first()
        .and_then(|opt| opt.value.as_str())
        .unwrap_or("")
        .trim();

    if code.is_empty() {
        send_error(context, interaction, "Please provide code to execute.", None).await;
        return;
    }

    // Use the execute function from core to evaluate the input
    let result = match crate::core::execute(code, &mut session.variables) {
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
    session.history.push(code.to_string());
    
    // Create description based on result
    let description = match result {
        Some(val) => format!(
            "**Code:**\n```rs\n{}\n```\n\
            **Result:**\n```rs\n{}\n```",
            code.trim(),
            val
        ),
        None => format!(
            "**Code:**\n```rs\n{}\n```\n",
            code.trim()
        )
    };

    // Create response embed
    let embed = CreateEmbed::new()
        .title("Code Execution Successful")
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
        error!("Failed to respond to execute command: {:?}", error);
    }
}

/// Handles the context menu command for executing code from messages.
/// 
/// Maintains the original formatting and executes the code while showing
/// both input and output.
pub async fn handle_execute_code(
    context: &Context,
    interaction: &CommandInteraction,
    session: &mut UserSession,
    code: &str,
) {
    if code.is_empty() {
        send_error(context, interaction, "The extracted code is empty.", None).await;
        return;
    }

    // Use the execute function from core to evaluate the input
    let result = match crate::core::execute(code, &mut session.variables) {
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
    session.history.push(code.to_string());
    
    // Create description based on result
    let description = match result {
        Some(val) => format!(
            "**Input:**\n```rs\n{}\n```\n\
            **Result:**\n```rs\n{}\n```",
            code.trim(),
            val
        ),
        None => format!(
            "**Input:**\n```rs\n{}\n```\n",
            code.trim()
        )
    };

    // Create response embed
    let embed = CreateEmbed::new()
        .title("Code Execution Successful")
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
        error!("Failed to respond to execute code command: {:?}", error);
    }
} 