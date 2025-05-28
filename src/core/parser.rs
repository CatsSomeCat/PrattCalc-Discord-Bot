//! Parser module for converting tokens into an abstract syntax tree.
//!
//! This module implements a recursive descent parser with Pratt parsing for
//! expressions. It converts a stream of tokens from the lexical analyzer into
//! an abstract syntax tree (AST) that can be evaluated or executed.

use crate::core::lexical_analyzer::{Token, Tokenizer};
use crate::core::ast_expression::Expression;
use crate::core::ast_statement::Statement;
use crate::core::error_types::ParseError;

/// A parser that converts tokens into an abstract syntax tree.
#[derive(Clone)]
pub struct Parser {
    /// The tokenizer providing the token stream.
    tokenizer: Tokenizer,
}

impl Parser {
    /// Creates a new parser with the given tokenizer.
    pub fn new(tokenizer: Tokenizer) -> Self {
        Self { tokenizer }
    }
    
    /// Parses the input as an expression.
    pub fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        let expr = Expression::parse(&mut self.tokenizer, 0.0)?;
        
        // Ensure we've consumed all tokens
        if self.tokenizer.peek_token() != &Token::EndOfInput {
            return Err(ParseError::UnexpectedToken(
                format!("Expected end of input, found {:?}", self.tokenizer.peek_token())
            ));
        }
        
        Ok(expr)
    }
    
    /// Parses the input as a statement or sequence of statements.
    pub fn parse_statements(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut statements = Vec::new();
        
        while self.tokenizer.peek_token() != &Token::EndOfInput {
            let statement = Statement::parse(&mut self.tokenizer)?;
            statements.push(statement);
            
            // Skip any semicolons between statements
            while self.tokenizer.peek_token() == &Token::Operator(';') {
                self.tokenizer.next_token();
            }
        }
        
        Ok(statements)
    }
    
    /// Tries to parse a single statement.
    pub fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        Statement::parse(&mut self.tokenizer)
    }
    
    /// Tries to parse the input first as statements, then as an expression.
    ///
    /// This method attempts to parse the input as a sequence of statements. If that
    /// fails, it falls back to parsing it as a single expression.
    pub fn parse_program(&mut self) -> Result<ParsedProgram, ParseError> {
        // First try parsing as statements
        let mut tmp_parser = self.clone();
        match tmp_parser.parse_statements() {
            Ok(statements) if !statements.is_empty() => {
                // If successful, update our state and return the statements
                *self = tmp_parser;
                return Ok(ParsedProgram::Statements(statements));
            }
            _ => {
                // Reset the tokenizer and try parsing as expression
                self.tokenizer.reset();
                match self.parse_expression() {
                    Ok(expr) => Ok(ParsedProgram::Expression(expr)),
                    Err(err) => Err(err),
                }
            }
        }
    }
}

/// Represents the result of parsing a program.
pub enum ParsedProgram {
    /// Statements from a script.
    Statements(Vec<Statement>),
    
    /// Single expression.
    Expression(Expression),
} 

//=============================================================================
// Helper functions for parsing from strings
//=============================================================================

/// Parses a string input as an expression.
///
/// This is a convenience function that creates a tokenizer and parser,
/// then parses the input as an expression.
pub fn parse_expression(input: &str) -> Result<Expression, ParseError> {
    let tokenizer = Tokenizer::from_input(input);
    let mut parser = Parser::new(tokenizer);
    parser.parse_expression()
}

/// Parses a string input as a program (statements or expression).
///
/// This is a convenience function that creates a tokenizer and parser,
/// then parses the input as a program.
pub fn parse_program(input: &str) -> Result<ParsedProgram, ParseError> {
    let tokenizer = Tokenizer::from_input(input);
    let mut parser = Parser::new(tokenizer);
    parser.parse_program()
} 
