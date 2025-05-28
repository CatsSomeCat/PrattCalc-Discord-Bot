// Core module entry point that re-exports all components

// Submodules
mod lexical_analyzer;
mod ast_expression;
mod ast_statement;
mod symbol_manager;
mod parser;
mod interpreter;
mod error_types;
mod execution_state;

// Re-exports for public API
pub use lexical_analyzer::Tokenizer;
pub use ast_expression::Expression;
pub use ast_statement::Statement;
pub use symbol_manager::SymbolTable;
pub use parser::Parser;
pub use interpreter::{evaluate, execute};
pub use execution_state::ExitState;
pub use error_types::{ParseError, EvalError, ExecutionError, InterpreterError};

/// Type alias for calculator errors.
pub type CalcError = InterpreterError; 
