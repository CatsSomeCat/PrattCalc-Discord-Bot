use std::collections::HashSet;

use crate::core::lexical_analyzer::{Token, Tokenizer};
use crate::core::ast_expression::Expression;
use crate::core::symbol_manager::SymbolTable;
use crate::core::error_types::{ParseError, EvalError, SymbolError, ControlFlowError};
use crate::core::execution_state::with_exit_state;

/// Statement types in the language.
#[derive(Clone)]
pub enum Statement {
    /// An expression used as a statement.
    Expression(Expression),

    /// A block of statements.
    Block(Vec<Statement>),

    /// An if-else conditional statement.
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },

    /// A while loop.
    While {
        condition: Expression,
        body: Box<Statement>,
    },

    /// A break statement.
    Break,

    /// A continue statement.
    Continue,

    /// A return statement with optional value.
    Return(Option<Expression>),
    
    /// An end statement with optional value.
    End(Option<Expression>),

    /// A variable declaration with optional initializer.
    Let {
        name: String,
        initializer: Option<Expression>,
    },
    
    /// A constant declaration with required initializer.
    Const {
        name: String,
        initializer: Expression,
    },
}

// Add this enum to track control flow state between nested structures
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ControlFlow {
    Normal,
    Break,
    Continue,
    Return,
}

impl Statement {
    /// Parse a single statement from the token stream.
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Statement, ParseError> {
        if tokenizer.peek_token() == &Token::EndOfInput {
            return Err(ParseError::EmptyInput);
        }
        
        // Clone here to avoid borrow checker issues
        let statement = match tokenizer.peek_token().clone() {
            Token::Keyword(keyword) => {
                tokenizer.next_token(); // consume keyword
                match keyword.as_str() {
                    "if" => Self::parse_if_statement(tokenizer)?,
                    "while" => Self::parse_while_statement(tokenizer)?,
                    "break" => Statement::Break,
                    "continue" => Statement::Continue,
                    "return" => Self::parse_return_statement(tokenizer)?,
                    "let" => Self::parse_let_statement(tokenizer)?,
                    "const" => Self::parse_const_statement(tokenizer)?,
                    "end" => Self::parse_end_statement(tokenizer)?,
                    _ => return Err(ParseError::UnexpectedToken(keyword)),
                }
            }
            Token::Operator('{') => Self::parse_block_statement(tokenizer)?,
            _ => {
                // If not a keyword or block, treat as expression statement
                let expression = Expression::parse(tokenizer, 0.0)?;
                Statement::Expression(expression)
            }
        };

        // Skip any trailing semicolon
        if tokenizer.peek_token() == &Token::Operator(';') {
            tokenizer.next_token();
        }

        Ok(statement)
    }
    
    /// Parse a block statement.
    fn parse_block_statement(tokenizer: &mut Tokenizer) -> Result<Statement, ParseError> {
        tokenizer.next_token(); // consume '{'
        let mut statements = Vec::new();

        while tokenizer.peek_token() != &Token::Operator('}') {
            if tokenizer.peek_token() == &Token::EndOfInput {
                return Err(ParseError::ExpectedBlock);
            }

            // Skip empty statements (lone semicolons)
            if tokenizer.peek_token() == &Token::Operator(';') {
                tokenizer.next_token();
                continue;
            }

            // Parse the next statement
            let statement = Self::parse(tokenizer)?;
            statements.push(statement);
        }

        tokenizer.next_token(); // consume '}'
        Ok(Statement::Block(statements))
    }
    
    /// Parse an if statement.
    fn parse_if_statement(tokenizer: &mut Tokenizer) -> Result<Statement, ParseError> {
        let condition = Expression::parse(tokenizer, 0.0)?;
        
        // Skip any semicolons after the condition
        while tokenizer.peek_token() == &Token::Operator(';') {
            tokenizer.next_token();
        }

        // Handle the then branch
        let then_branch = match tokenizer.peek_token() {
            Token::Operator('{') => Box::new(Self::parse_block_statement(tokenizer)?),
            _ => {
                // If no block, parse a single statement
                Box::new(Self::parse(tokenizer)?)
            }
        };

        // Skip any semicolons after the then branch
        while tokenizer.peek_token() == &Token::Operator(';') {
            tokenizer.next_token();
        }

        // Handle the else branch
        let else_branch = if let Token::Keyword(keyword) = tokenizer.peek_token() {
            if keyword == "else" {
                tokenizer.next_token(); // consume 'else'
                match tokenizer.peek_token() {
                    Token::Operator('{') => Some(Box::new(Self::parse_block_statement(tokenizer)?)),
                    Token::Keyword(ref kw) if kw == "if" => {
                        tokenizer.next_token(); // consume 'if'
                        Some(Box::new(Self::parse_if_statement(tokenizer)?))
                    }
                    _ => Some(Box::new(Self::parse(tokenizer)?))
                }
            } else {
                None
            }
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }
    
    /// Parse a while statement.
    fn parse_while_statement(tokenizer: &mut Tokenizer) -> Result<Statement, ParseError> {
        let condition = Expression::parse(tokenizer, 0.0)?;
        
        // Skip any semicolons after the condition
        while tokenizer.peek_token() == &Token::Operator(';') {
            tokenizer.next_token();
        }

        // Handle the body
        let body = match tokenizer.peek_token() {
            Token::Operator('{') => Box::new(Self::parse_block_statement(tokenizer)?),
            _ => {
                // If no block, parse a single statement
                Box::new(Self::parse(tokenizer)?)
            }
        };

        Ok(Statement::While {
            condition,
            body,
        })
    }
    
    /// Parse a return statement.
    fn parse_return_statement(tokenizer: &mut Tokenizer) -> Result<Statement, ParseError> {
        let expression = if tokenizer.peek_token() == &Token::EndOfInput || tokenizer.peek_token() == &Token::Operator(';') {
            None
        } else {
            Some(Expression::parse(tokenizer, 0.0)?)
        };
        Ok(Statement::Return(expression))
    }
    
    /// Parse an end statement.
    fn parse_end_statement(tokenizer: &mut Tokenizer) -> Result<Statement, ParseError> {
        let expression = if tokenizer.peek_token() == &Token::EndOfInput || tokenizer.peek_token() == &Token::Operator(';') {
            None
        } else {
            Some(Expression::parse(tokenizer, 0.0)?)
        };
        Ok(Statement::End(expression))
    }
    
    /// Parse a let statement.
    fn parse_let_statement(tokenizer: &mut Tokenizer) -> Result<Statement, ParseError> {
        let name = match tokenizer.next_token() {
            Token::Literal(name) => name,
            token => return Err(ParseError::UnexpectedToken(format!("{:?}", token))),
        };

        let initializer = if let Token::Operator('=') = tokenizer.peek_token() {
            tokenizer.next_token(); // consume '='
            Some(Expression::parse(tokenizer, 0.0)?)
        } else {
            None
        };

        // Skip any trailing semicolon
        if tokenizer.peek_token() == &Token::Operator(';') {
            tokenizer.next_token();
        }

        Ok(Statement::Let { name, initializer })
    }

    /// Parse a const statement.
    fn parse_const_statement(tokenizer: &mut Tokenizer) -> Result<Statement, ParseError> {
        let name = match tokenizer.next_token() {
            Token::Literal(name) => name,
            token => return Err(ParseError::UnexpectedToken(format!("{:?}", token))),
        };

        // For const declaration, '=' is required followed by initializer
        if tokenizer.peek_token() != &Token::Operator('=') {
            return Err(ParseError::ExpectedOperator("=".to_string()));
        }
        
        tokenizer.next_token(); // consume '='
        let initializer = Expression::parse(tokenizer, 0.0)?;

        // Skip any trailing semicolon
        if tokenizer.peek_token() == &Token::Operator(';') {
            tokenizer.next_token();
        }

        Ok(Statement::Const { name, initializer })
    }

    /// Evaluate a statement in the given context.
    pub fn evaluate(&self, context: &mut SymbolTable<f32>) -> Result<(Option<f32>, ControlFlow), EvalError> {
        match self {
            Statement::Expression(expr) => {
                // If this is an assignment, check if the variable exists before evaluation
                if let Expression::Operation('=', operands) = expr {
                    if let Expression::Literal(var_name) = &operands[0] {
                        // Check if variable exists before assignment
                        if !context.contains(var_name) {
                            return Err(SymbolError::UndeclaredVariable(var_name.clone()).into());
                        }
                        
                        // Variable exists, evaluate and update
                        let value = operands[1].evaluate(context)?;
                        context.set_variable(var_name.clone(), value)?;
                        return Ok((Some(value), ControlFlow::Normal));
                    }
                }
                
                // Not an assignment or handled above
                let value = expr.evaluate(context)?;
                Ok((Some(value), ControlFlow::Normal))
            }

            Statement::Block(statements) => {
                // Create a new scope by cloning the current context
                let mut block_context = context.new_scope();
                let mut last_value = None;
                let mut control_flow = ControlFlow::Normal;

                // Keep track of variables defined in this block
                let mut block_vars = HashSet::new();

                // Evaluate each statement in the block with the new context
                for statement in statements {
                    // Track any new variables defined by let statements
                    if let Statement::Let { name, .. } = statement {
                        block_vars.insert(name.clone());
                    } else if let Statement::Const { name, .. } = statement {
                        block_vars.insert(name.clone());
                    }
                    
                    // Evaluate the current statement
                    let (value, stmt_flow) = statement.evaluate(&mut block_context)?;
                    
                    // Update the last value if one was returned
                    if let Some(v) = value {
                        last_value = Some(v);
                    }
                    
                    // Handle control flow
                    if stmt_flow != ControlFlow::Normal {
                        control_flow = stmt_flow;
                        break;
                    }
                }

                // Copy back only variables that were not defined in this block
                for (key, value) in block_context.values.iter() {
                    // Skip variables defined in this block (including shadowed ones)
                    if block_vars.contains(key) {
                        continue;
                    }
                    
                    // Skip variables that haven't changed
                    if context.get(key) == Some(value) {
                        continue;
                    }
                    
                    // Don't modify constants from the parent scope
                    if context.is_constant(key) {
                        continue;
                    }
                    
                    // Only update variables that already exist in the outer scope
                    if context.contains(key) {
                        context.set_variable(key.clone(), value.clone())?;
                    }
                }

                Ok((last_value, control_flow))
            }

            Statement::If { condition, then_branch, else_branch } => {
                let condition_value = condition.evaluate(context)?;
                if condition_value != 0.0 {
                    // Create a new scope for the then branch
                    let mut then_context = context.new_scope();
                    let (result, control_flow) = then_branch.evaluate(&mut then_context)?;

                    // Track variables defined in this block to avoid shadowing issues
                    let mut defined_vars = HashSet::new();
                    if let Statement::Block(statements) = &**then_branch {
                        for stmt in statements {
                            if let Statement::Let { name, .. } = stmt {
                                defined_vars.insert(name.clone());
                            } else if let Statement::Const { name, .. } = stmt {
                                defined_vars.insert(name.clone());
                            }
                        }
                    }

                    // Copy variables from the then branch back to the parent context
                    for (key, value) in then_context.values.iter() {
                        // Skip variables defined in this block (including shadowed ones)
                        if defined_vars.contains(key) {
                            continue;
                        }
                        
                        // Skip variables that haven't changed
                        if context.get(key) == Some(value) {
                            continue;
                        }
                        
                        // Don't modify constants from the parent scope
                        if context.is_constant(key) {
                            continue;
                        }
                        
                        // Only update variables that already exist in the outer scope
                        if context.contains(key) {
                            context.set_variable(key.clone(), value.clone())?;
                        }
                    }
                    
                    Ok((result, control_flow))
                } else if let Some(else_br) = else_branch {
                    // Create a new scope for the else branch
                    let mut else_context = context.new_scope();
                    let (result, control_flow) = else_br.evaluate(&mut else_context)?;

                    // Track variables defined in this block to avoid shadowing issues
                    let mut defined_vars = HashSet::new();
                    if let Statement::Block(statements) = &**else_br {
                        for stmt in statements {
                            if let Statement::Let { name, .. } = stmt {
                                defined_vars.insert(name.clone());
                            } else if let Statement::Const { name, .. } = stmt {
                                defined_vars.insert(name.clone());
                            }
                        }
                    }

                    // Copy variables from the else branch back to the parent context
                    for (key, value) in else_context.values.iter() {
                        // Skip variables defined in this block (including shadowed ones)
                        if defined_vars.contains(key) {
                            continue;
                        }
                        
                        // Skip variables that haven't changed
                        if context.get(key) == Some(value) {
                            continue;
                        }
                        
                        // Don't modify constants from the parent scope
                        if context.is_constant(key) {
                            continue;
                        }
                        
                        // Only update variables that already exist in the outer scope
                        if context.contains(key) {
                            context.set_variable(key.clone(), value.clone())?;
                        }
                    }
                    
                    Ok((result, control_flow))
                } else {
                    Ok((Some(0.0), ControlFlow::Normal))
                }
            }

            Statement::While { condition, body } => {
                let mut last_value = None;
                while condition.evaluate(context)? != 0.0 {
                    // Create a new scope for each iteration
                    let mut loop_context = context.new_scope();
                    
                    // Evaluate the body with control flow information
                    let (value, control_flow) = body.evaluate(&mut loop_context)?;
                    
                    // Update the last value if one was returned
                    if let Some(v) = value {
                        last_value = Some(v);
                    }

                    // Track variables defined in this block to avoid shadowing issues
                    let mut defined_vars = HashSet::new();
                    if let Statement::Block(statements) = &**body {
                        for stmt in statements {
                            if let Statement::Let { name, .. } = stmt {
                                defined_vars.insert(name.clone());
                            } else if let Statement::Const { name, .. } = stmt {
                                defined_vars.insert(name.clone());
                            }
                        }
                    }

                    // Copy variables from the loop iteration back to the parent context
                    for (key, value) in loop_context.values.iter() {
                        // Skip variables defined in this block (including shadowed ones)
                        if defined_vars.contains(key) {
                            continue;
                        }
                        
                        // Skip variables that haven't changed
                        if context.get(key) == Some(value) {
                            continue;
                        }
                        
                        // Don't modify constants from the parent scope
                        if context.is_constant(key) {
                            continue;
                        }
                        
                        // Only update variables that already exist in the outer scope
                        if context.contains(key) {
                            context.set_variable(key.clone(), value.clone())?;
                        }
                    }
                    
                    // Handle control flow instructions
                    match control_flow {
                        ControlFlow::Break => break,
                        ControlFlow::Continue => continue,
                        ControlFlow::Return => return Ok((last_value, ControlFlow::Return)),
                        ControlFlow::Normal => {}
                    }
                }
                
                Ok((last_value, ControlFlow::Normal))
            }

            Statement::Break => Ok((None, ControlFlow::Break)),
            
            Statement::Continue => Ok((None, ControlFlow::Continue)),
            
            Statement::Return(_expr) => {
                // Return is no longer functional, but we keep the syntax
                return Err(EvalError::ControlFlowError(
                    ControlFlowError::UnimplementedFeature("The 'return' statement is no longer functional. Use 'end' instead.".to_string())
                ));
            }

            Statement::Let { name, initializer } => {
                let value = if let Some(init) = initializer {
                    init.evaluate(context)?
                } else {
                    0.0
                };

                // If we're in a block scope and the variable already exists in the parent scope,
                // only update it in the current scope
                let is_block_scope = context.contains(name.as_str());
                if is_block_scope {
                    context.set_variable(name.clone(), value)?;
                } else {
                    // Otherwise, create a new variable in the current scope
                    context.set_variable(name.clone(), value)?;
                }

                Ok((Some(value), ControlFlow::Normal))
            }

            Statement::Const { name, initializer } => {
                let value = initializer.evaluate(context)?;
                context.declare_constant(name.clone(), value)?;
                Ok((Some(value), ControlFlow::Normal))
            }

            Statement::End(expr) => {
                let value = match expr {
                    Some(expr) => Some(expr.evaluate(context)?),
                    None => None,
                };
                
                // Set the exit state
                with_exit_state(|state| {
                    state.occurred = true;
                    state.value = value;
                });
                
                Ok((value, ControlFlow::Return))
            }
        }
    }
} 
