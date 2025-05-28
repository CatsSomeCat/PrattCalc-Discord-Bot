use crate::core::error_types::{InterpreterError, ExecutionError, ControlFlowError};
// use crate::core::ast_expression::Expression;
// use crate::core::ast_statement::Statement;
use crate::core::ast_statement::ControlFlow;
use crate::core::symbol_manager::SymbolTable;
use crate::core::execution_state::with_exit_state;
use crate::core::parser::{parse_expression, parse_program, ParsedProgram};

//=============================================================================
// Expression evaluation (pure calculations)
//=============================================================================

/// Evaluates a single arithmetic expression.
///
/// Takes an input string and evaluates it using the provided symbol table.
pub fn evaluate(input: &str, context: &SymbolTable<f32>) -> Result<f32, InterpreterError> {
    // Parse as an expression using the parser module
    let expr = match parse_expression(input) {
        Ok(expr) => expr,
        Err(err) => return Err(InterpreterError::Parse(err)),
    };
    
    // Evaluate the expression
    match expr.evaluate(context) {
        Ok(result) => Ok(result),
        Err(err) => Err(InterpreterError::Eval(err)),
    }
}

//=============================================================================
// Script execution (expressions, statements, control flow, etc.)
//=============================================================================

/// Executes a script or code block with statements and expressions.
/// 
/// Handles variable declarations, control flow, and other language features.
pub fn execute(input: &str, context: &mut SymbolTable<f32>) -> Result<Option<f32>, InterpreterError> {
    // Reset exit state at the start of execution
    with_exit_state(|state| {
        *state = Default::default();
    });

    // Parse program using the parser module
    match parse_program(input) {
        Ok(ParsedProgram::Statements(statements)) => {
            // Execute the statements
            let mut last_value: Option<f32> = None;
            let mut result = Ok(None);
            
            for statement in statements {
                // Check if an exit statement has been processed
                if with_exit_state(|state| state.occurred) {
                    break;
                }
                
                match statement.evaluate(context) {
                    Ok((value, control_flow)) => {
                        last_value = value;
                        
                        // Handle control flow outside proper context
                        match control_flow {
                            ControlFlow::Break => {
                                result = Err(ExecutionError::EvaluationError(
                                    ControlFlowError::BreakOutsideLoop.into()
                                ));
                                break;
                            },
                            ControlFlow::Continue => {
                                result = Err(ExecutionError::EvaluationError(
                                    ControlFlowError::ContinueOutsideLoop.into()
                                ));
                                break;
                            },
                            _ => {}
                        }
                    },
                    Err(error) => {
                        result = Err(ExecutionError::EvaluationError(error));
                        break;
                    }
                }
            }
            
            // If no errors occurred, update the result with the last value
            if result.is_ok() {
                result = Ok(last_value);
            }
            
            // Check exit state to determine what to return
            let exit_occurred = with_exit_state(|state| state.occurred);
            let exit_value = with_exit_state(|state| state.value);
            
            match (result, exit_occurred) {
                (Ok(Some(value)), false) => {
                    // If this is a variable in the global scope and nothing else was executed,
                    // return its value
                    Ok(Some(value))
                },
                (Ok(_), true) => {
                    // If an end statement was executed, return its value
                    Ok(exit_value)
                },
                (Ok(last_value), false) => {
                    // Normal execution completed without an end statement
                    Ok(last_value)
                },
                (Err(e), _) => match e {
                    ExecutionError::EvaluationError(eval_err) => Err(InterpreterError::Eval(eval_err)),
                    _ => Err(InterpreterError::Exec(e)),
                },
            }
        },
        Ok(ParsedProgram::Expression(expr)) => {
            // Execute as a single expression
            match expr.evaluate(context) {
                Ok(value) => Ok(Some(value)),
                Err(err) => Err(InterpreterError::Eval(err)),
            }
        },
        Err(err) => Err(InterpreterError::Parse(err)),
    }
}
