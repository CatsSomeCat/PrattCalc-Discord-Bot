use log::warn;
use serenity::all::*;
use serenity::builder::{CreateEmbed};

/// Enum representing different error categories for better organization and clearer user feedback.
#[derive(Debug, Clone, Copy)]
pub enum ErrorCategory {
    /// Syntax or parsing errors with expressions.
    Syntax,

    /// Errors during the execution or computation.
    Runtime,

    /// Errors with variable access or modification.
    Variable,

    /// Permission issues or Discord API errors.
    System,

    /// General or uncategorized errors.
    General,
}

impl ErrorCategory {
    /// Get the title string for this error category.
    pub fn title(&self) -> &'static str {
        match self {
            ErrorCategory::Syntax => "Syntax Error",
            ErrorCategory::Runtime => "Calculation Error",
            ErrorCategory::Variable => "Variable Error",
            ErrorCategory::System => "System Error",
            ErrorCategory::General => "Error",
        }
    }
    
    /// Get the color for this error category.
    pub fn color(&self) -> Colour {
        match self {
            ErrorCategory::Syntax => Colour::from_rgb(230, 126, 34),  // Orange
            ErrorCategory::Runtime => Colour::from_rgb(231, 76, 60),  // Red
            ErrorCategory::Variable => Colour::from_rgb(155, 89, 182), // Purple
            ErrorCategory::System => Colour::from_rgb(52, 152, 219),  // Blue
            ErrorCategory::General => Colour::DARK_RED,               // Default red
        }
    }
    
    /// Get a suggestion based on the error category.
    pub fn suggestion(&self) -> &'static str {
        match self {
            ErrorCategory::Syntax => "Check your expression for typos or missing parentheses.",
            ErrorCategory::Runtime => "Verify your calculation doesn't involve division by zero or other invalid operations.",
            ErrorCategory::Variable => "Make sure variables are declared before use and constants aren't being modified.",
            ErrorCategory::System => "Try again later or contact the bot administrator.",
            ErrorCategory::General => "Try simplifying your input or check the help command.",
        }
    }
    
    /// Try to determine error category from message content.
    pub fn from_message(message: &str) -> Self {
        if message.contains("Parser error") || message.contains("syntax") {
            ErrorCategory::Syntax
        } else if message.contains("division by zero") || message.contains("Invalid exponentiation") {
            ErrorCategory::Runtime
        } else if message.contains("Variable") || message.contains("constant") {
            ErrorCategory::Variable
        } else {
            ErrorCategory::General
        }
    }
}

/// Enhanced error handling utility that provides rich, categorized error information.
/// 
/// Displays errors with appropriate formatting, color coding, and helpful suggestions
/// based on error category. Also logs detailed information for debugging.
pub async fn send_error(
    context: &Context,
    interaction: &CommandInteraction,
    message: &str,
    category: Option<ErrorCategory>,
) {
    // Determine error category based on message content if not provided
    let category = category.unwrap_or_else(|| ErrorCategory::from_message(message));
    
    // Log the error with category for debugging
    warn!("{} - {}", category.title(), message);
    
    // Create an enhanced embed with appropriate styling and suggestions
    let embed = CreateEmbed::new()
        .title(category.title())
        .description(message)
        .field("Suggestion", category.suggestion(), false)
        .colour(category.color());
    
    // Attempt to send the response
    if let Err(error) = interaction.create_response(
        &context.http,
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().embed(embed)
        )
    ).await {
        log::error!("Failed to send error message: {}", error);
    }
} 