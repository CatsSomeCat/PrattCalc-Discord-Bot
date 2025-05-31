use std::fmt;
use crate::core::lexical_analyzer::{Token, Tokenizer};
use crate::core::error_types::{ParseError, EvalError, MathError, SymbolError, ControlFlowError};
use crate::core::symbol_manager::{SymbolTable, global_constants};
use crate::core::ast_statement::{Statement, ControlFlow};
use rand::Rng;

/// AST node for expressions.
///
/// Represents a node in the abstract syntax tree for expressions.
///
/// The Expression enum can be a literal value, an operation with operands,
/// or a function call with arguments.
#[derive(Clone, Debug)]
pub enum Expression {
    /// A literal string: numeric or variable identifier.
    Literal(String),

    /// An operation: operator character and operand subexpressions.
    /// 
    /// The first element in the Vec is the left-hand operand for binary operators,
    /// or the only operand for unary operators.
    Operation(char, Vec<Expression>),

    /// A function call: function name and argument expressions.
    FunctionCall(String, Vec<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal(value) => write!(formatter, "{}", value),
            Expression::Operation(operator, operands) => {
                write!(formatter, "({}", operator)?;
                for operand in operands {
                    write!(formatter, " {}", operand)?;
                }
                write!(formatter, ")")
            }
            Expression::FunctionCall(name, args) => {
                write!(formatter, "{}(", name)?;
                for (index, arg_expr) in args.iter().enumerate() {
                    if index > 0 {
                        write!(formatter, ", ")?;
                    }
                    write!(formatter, "{}", arg_expr)?;
                }
                write!(formatter, ")")
            }
        }
    }
}

impl Expression {
    /// Parses an expression from a tokenizer with aminimum binding power.
    ///
    /// This is the core of the Pratt parsing algorithm.
    ///
    /// It uses binding power (precedence) to determine how expressions should be grouped.
    pub fn parse(tokenizer: &mut Tokenizer, min_bp: f32) -> Result<Self, ParseError> {
        // Phase 1: Parse the left-hand side (LHS) operand or prefix expression
        let mut lhs = match tokenizer.peek_token() {
            // Keywords are not allowed in expressions, but we'll handle them in statement parsing
            Token::Keyword(_) => {
                let statement = Statement::parse(tokenizer)?;
                match statement {
                    Statement::Expression(expr) => expr,
                    _ => return Err(ParseError::UnexpectedToken(format!("{:?}", tokenizer.peek_token()))),
                }
            },

            // Grouped expression; parse expressions inside parentheses
            Token::Operator('(') => {
                tokenizer.next_token(); // consume '('
                let expr = Self::parse(tokenizer, 0.0)?;
                match tokenizer.next_token() {
                    Token::Operator(')') => expr,
                    _ => return Err(ParseError::UnmatchedParenthesis),
                }
            }

            // Prefix operator or unary/root expression (e.g., -a, √a, a √ b)
            Token::Operator(op) if prefix_binding_power(*op).is_some() => {
                let prefix_op = *op;
                tokenizer.next_token(); // consume operator
                let binding_power = prefix_binding_power(prefix_op).unwrap();

                // Parse the operand following the prefix operator
                let first_operand = Self::parse(tokenizer, binding_power)?;
                let mut operands = vec![first_operand];

                // Special case for √ operator that may accept a second operand (e.g., a √ b)
                if prefix_op == '√' && matches!(
                    tokenizer.peek_token(),
                    Token::Literal(_) | Token::Operator('(') | Token::Operator('√')
                ) {
                    operands.push(Self::parse(tokenizer, binding_power)?);
                }

                Expression::Operation(prefix_op, operands)
            }

            // Literal token
            Token::Literal(_) => {
                if let Token::Literal(lit) = tokenizer.next_token() {
                    // Check for function call (literal followed by open parenthesis)
                    if tokenizer.peek_token() == &Token::Operator('(') {
                        tokenizer.next_token(); // consume '('
                        let mut args = Vec::new();

                        // Parse argument list
                        if tokenizer.peek_token() != &Token::Operator(')') {
                            loop {
                                args.push(Self::parse(tokenizer, 0.0)?);
                                if tokenizer.peek_token() == &Token::Operator(',') {
                                    tokenizer.next_token(); // consume ','
                                } else {
                                    break;
                                }
                            }
                        }
                        
                        // Ensure closing parenthesis
                        if tokenizer.peek_token() != &Token::Operator(')') {
                            return Err(ParseError::UnmatchedParenthesis);
                        }
                        tokenizer.next_token(); // consume ')'
                        
                        Expression::FunctionCall(lit, args)
                    } else {
                        Expression::Literal(lit)
                    }
                } else {
                    unreachable!("Expected literal after Token::Literal")
                }
            }

            // Any unexpected token at the beginning of an expression
            unexpected => return Err(ParseError::UnexpectedToken(format!("{:?}", unexpected))),
        };

        // Phase 2: Parse infix and augmented operators (while loop for right recursion)
        // This continues grabbing operators and right-hand expressions as long as
        // the precedence/binding power is sufficient
        loop {
            match tokenizer.peek_token() {
                // End of expression or expression group
                Token::EndOfInput | Token::Operator(')') | Token::Operator(',') | Token::Operator(';') => break,

                // Infix operators (e.g., +, -, *, /, ^, etc.)
                Token::Operator(op) if infix_binding_power(*op).is_some() => {
                    let (left_bp, right_bp, is_left_associative) = infix_binding_power(*op).unwrap();

                    // Stop parsing if operator precedence is lower than current context
                    // The binding power check ensures proper precedence handling
                    if (is_left_associative && left_bp < min_bp)
                        || (!is_left_associative && left_bp <= min_bp)
                    {
                        break;
                    }
                    
                    let operator = *op;
                    tokenizer.next_token(); // consume operator
                    
                    // Recursively parse the right-hand side with the appropriate binding power
                    let rhs = Self::parse(tokenizer, right_bp)?;
                    
                    // Combine the left and right expressions with the operator
                    lhs = Expression::Operation(operator, vec![lhs, rhs]);
                }

                // Augmented assignment (e.g., +=, -=, *=, and etc.)
                Token::AugAssign(_) => {
                    if let Token::AugAssign(aug_op_str) = tokenizer.next_token() {
                        // Extract actual operator from the augmented assignment (e.g., "+=" -> '+')
                        let base_op = aug_op_str.chars().next().unwrap();
                        let rhs = Self::parse(tokenizer, 0.0)?;

                        // Desugar x += y => x = x + y
                        // The actual check for variable existence will happen during evaluation
                        lhs = Expression::Operation(
                            '=',
                            vec![lhs.clone(), Expression::Operation(base_op, vec![lhs, rhs])],
                        );
                    }
                }

                // Stop parsing current expression
                _ => break,
            }
        }

        Ok(lhs)
    }

    /// Identifies if this is an assignment operation.
    #[allow(dead_code)]
    pub fn is_assignment(&self) -> Option<(String, Expression)> {
        if let Expression::Operation(op_char, operands) = self {
            if operands.len() == 2 {
                if let Expression::Literal(var_name) = &operands[0] {
                    if *op_char == '=' {
                        let rhs_expr = operands[1].clone();
                        return Some((var_name.clone(), rhs_expr));
                    }
                }
            }
        }
        None
    }

    /// Evaluate the AST node against a context of variable bindings.
    ///
    /// Recursively evaluates the expression using the provided SymbolTable for variable lookups.
    /// 
    /// For variable names, it first checks the local context (SymbolTable), then global constants.
    pub fn evaluate(&self, context: &SymbolTable<f32>) -> Result<f32, EvalError> {
        match self {
            // Literal: number or variable
            Expression::Literal(text) => {
                // Hexadecimal (0xFF)
                if let Some(hex_digits) = text.strip_prefix("0x") {
                    let value = u32::from_str_radix(hex_digits, 16)
                        .map_err(|_| MathError::InvalidExponentiation)? as f32;
                    return Ok(value);
                }
                // Binary (0b1010)
                if let Some(bin_digits) = text.strip_prefix("0b") {
                    let value = u32::from_str_radix(bin_digits, 2)
                        .map_err(|_| MathError::InvalidExponentiation)? as f32;
                    return Ok(value);
                }
                // Decimal literal
                if let Ok(decimal_value) = text.parse::<f32>() {
                    return Ok(decimal_value);
                }
                // Variable lookup - first check local context
                if let Some(value) = context.get(text) {
                    return Ok(*value);
                }
                // Then check global constants
                if let Some(value) = global_constants().get(text) {
                    return Ok(value);
                }
                // Not found anywhere
                Err(SymbolError::VariableNotFound(text.clone()).into())
            }

            // Infix or prefix operation (unary, binary, root)
            Expression::Operation(operator, operands) => {
                // Special handling for assignment operator
                if *operator == '=' && operands.len() == 2 {
                    if let Expression::Literal(var_name) = &operands[0] {
                        // Check if we're trying to assign to a global constant
                        if global_constants().contains(var_name) {
                            return Err(SymbolError::ImmutableConstant(var_name.clone()).into());
                        }
                    }
                }

                // Evaluate left operand (always present)
                let left_val = operands[0].evaluate(context)?;

                // Evaluate right operand if binary, otherwise default to 0.0
                let right_val = if operands.len() > 1 {
                    operands[1].evaluate(context)?
                } else {
                    0.0
                };

                match *operator {
                    // Arithmetic operations
                    '+' => Ok(left_val + right_val),
                    '-' if operands.len() == 1 => Ok(-left_val), // unary minus
                    '-' => Ok(left_val - right_val),
                    '*' => Ok(left_val * right_val),

                    // Division with zero check
                    '/' => {
                        if right_val == 0.0 {
                            Err(MathError::DivisionByZero.into())
                        } else {
                            Ok(left_val / right_val)
                        }
                    }

                    // Modulo with zero check
                    '%' => {
                        if right_val == 0.0 {
                            Err(MathError::ModuloByZero.into())
                        } else {
                            Ok(left_val % right_val)
                        }
                    }

                    // Exponentiation, check for invalid negative base + fractional exponent
                    '^' => {
                        if left_val < 0.0 && right_val.fract() != 0.0 {
                            Err(MathError::InvalidExponentiation.into())
                        } else {
                            Ok(left_val.powf(right_val))
                        }
                    }

                    // Root operation, expects exactly two operands
                    '√' => {
                        if operands.len() != 2 {
                            return Err(MathError::InvalidExponentiation.into());
                        }
                        let degree = left_val;
                        let radicand = right_val;
                        if degree == 0.0 {
                            Err(MathError::ZerothRoot.into())
                        } else if radicand < 0.0 && (1.0_f32 / degree).fract() != 0.0 {
                            Err(MathError::NegativeRoot.into())
                        } else {
                            Ok(radicand.powf(1.0 / degree))
                        }
                    }

                    // Logical operators
                    '&' => Ok(if left_val != 0.0 && right_val != 0.0 { 1.0 } else { 0.0 }), // AND
                    '|' => Ok(if left_val != 0.0 || right_val != 0.0 { 1.0 } else { 0.0 }), // OR
                    'x' => Ok(if (left_val != 0.0) != (right_val != 0.0) { 1.0 } else { 0.0 }), // XOR
                    'q' => Ok(if (left_val != 0.0) == (right_val != 0.0) { 1.0 } else { 0.0 }), // XNOR
                    'a' => Ok(if !(left_val != 0.0 && right_val != 0.0) { 1.0 } else { 0.0 }), // NAND
                    'o' => Ok(if !(left_val != 0.0 || right_val != 0.0) { 1.0 } else { 0.0 }), // NOR
                    '!' => Ok(if left_val == 0.0 { 1.0 } else { 0.0 }), // NOT (unary)

                    // Comparison operators
                    '>' => Ok(if left_val > right_val { 1.0 } else { 0.0 }),
                    '<' => Ok(if left_val < right_val { 1.0 } else { 0.0 }),
                    'g' => Ok(if left_val >= right_val { 1.0 } else { 0.0 }), // >=
                    'l' => Ok(if left_val <= right_val { 1.0 } else { 0.0 }), // <=
                    'e' => Ok(if (left_val - right_val).abs() < f32::EPSILON { 1.0 } else { 0.0 }), // ==
                    'n' => Ok(if (left_val - right_val).abs() >= f32::EPSILON { 1.0 } else { 0.0 }), // !=

                    // Dot-access operator, returns the right-hand side
                    '.' => Ok(right_val),

                    // Assignment operator
                    '=' => Ok(right_val),

                    // Unsupported operator
                    other => Err(MathError::UnsupportedOperator(other).into()),
                }
            }

            // Function call
            Expression::FunctionCall(name, args) => {
                // First, check if it's a procedure call
                if context.procedures.contains_key(name) {
                    // Return error - procedure calls must be handled as statements
                    return Err(ControlFlowError::UnimplementedFeature(
                        format!("Procedure '{}' cannot be called as a function expression", name)
                    ).into());
                }
                
                // Evaluate all arguments first
                let mut evaluated_args = Vec::with_capacity(args.len());
                for arg in args {
                    evaluated_args.push(arg.evaluate(context)?);
                }
                
                // Check for built-in functions first
                match name.as_str() {
                    "sin"   => Ok(evaluated_args[0].sin()),
                    "cos"   => Ok(evaluated_args[0].cos()),
                    "tan"   => Ok(evaluated_args[0].tan()),
                    // Additional trigonometric functions
                    "cot"   => {
                        let tan_val = evaluated_args[0].tan();
                        if tan_val == 0.0 {
                            Err(MathError::UnsupportedFunction("Division by zero in cotangent".to_string()).into())
                        } else {
                            Ok(1.0 / tan_val)
                        }
                    },
                    "sec"   => {
                        let cos_val = evaluated_args[0].cos();
                        if cos_val == 0.0 {
                            Err(MathError::UnsupportedFunction("Division by zero in secant".to_string()).into())
                        } else {
                            Ok(1.0 / cos_val)
                        }
                    },
                    "csc"   => {
                        let sin_val = evaluated_args[0].sin();
                        if sin_val == 0.0 {
                            Err(MathError::UnsupportedFunction("Division by zero in cosecant".to_string()).into())
                        } else {
                            Ok(1.0 / sin_val)
                        }
                    },
                    // Inverse trigonometric functions
                    "asin"  => Ok(evaluated_args[0].asin()),
                    "acos"  => Ok(evaluated_args[0].acos()),
                    "atan"  => Ok(evaluated_args[0].atan()),
                    "atan2" => {
                        if evaluated_args.len() != 2 {
                            return Err(MathError::UnsupportedFunction("atan2 requires two arguments: y, x".to_string()).into());
                        }
                        let y = evaluated_args[0];
                        let x = evaluated_args[1];
                        Ok(y.atan2(x))
                    },
                    "log"   => Ok(evaluated_args[0].ln()),
                    "sqrt"  => Ok(evaluated_args[0].sqrt()),
                    "abs"   => Ok(evaluated_args[0].abs()),
                    "max"   => Ok(evaluated_args[0].max(evaluated_args[1])),
                    "min"   => Ok(evaluated_args[0].min(evaluated_args[1])),
                    "rand"  => {
                        let mut rng = rand::thread_rng();
                        if evaluated_args.is_empty() {
                            // rand() with no args: returns a value between 0 and 1
                            Ok(rng.gen::<f32>())
                        } else if evaluated_args.len() == 1 {
                            // rand(max): returns a value between 0 and max
                            let max = evaluated_args[0];
                            Ok(rng.gen::<f32>() * max)
                        } else if evaluated_args.len() == 2 {
                            // rand(min, max): returns a value between min and max
                            let min = evaluated_args[0];
                            let max = evaluated_args[1];
                            if min >= max {
                                return Err(MathError::UnsupportedFunction("min must be less than max".to_string()).into());
                            }
                            Ok(rng.gen_range(min..max))
                        } else {
                            Err(MathError::UnsupportedFunction("rand() accepts 0, 1, or 2 arguments".to_string()).into())
                        }
                    },
                    // If not a built-in function, check for user-defined functions
                    _ => {
                        if let Some((params, body)) = context.get_function(name) {
                            // Create a new scope for function execution
                            let mut function_scope = context.new_scope();
                            
                            // Check argument count matches parameter count
                            if evaluated_args.len() != params.len() {
                                return Err(ControlFlowError::WrongArgumentCount {
                                    name: name.clone(),
                                    expected: params.len(),
                                    got: evaluated_args.len(),
                                }.into());
                            }
                            
                            // Bind evaluated arguments to parameters
                            for (i, &arg_value) in evaluated_args.iter().enumerate() {
                                function_scope.set_variable(params[i].clone(), arg_value)?;
                            }
                            
                            // Execute the function body
                            match body.evaluate(&mut function_scope)? {
                                (Some(value), ControlFlow::Return) => Ok(value),
                                (Some(value), _) => Ok(value),  // Return the last value if no explicit return
                                (None, _) => Ok(0.0),  // Default return value if none specified
                            }
                        } else {
                            Err(ControlFlowError::FunctionOrProcedureNotFound {
                                name: name.clone(),
                            }.into())
                        }
                    }
                }
            }
        }
    }
}

/// Defines precedence and associativity.
///
/// Returns a tuple of (left_binding_power, right_binding_power, is_left_associative).
/// 
/// Higher binding power means higher precedence.
pub fn infix_binding_power(op: char) -> Option<(f32, f32, bool)> {
    // For left associative operators, left_bp < right_bp
    // For right associative operators, left_bp > right_bp
    match op {
        '=' => Some((0.2, 0.1, false)),        // right-associative
        '&' | '|' | 'x' | 'q' | 'a' | 'o' => Some((0.3, 0.4, true)), // logical operators
        '<' | '>' | 'g' | 'l' | 'e' | 'n' => Some((0.5, 0.6, true)), // comparison operators
        '+' | '-' => Some((1.0, 1.1, true)),   // left-associative
        '*' | '/' | '%' => Some((2.0, 2.1, true)),
        '.' => Some((5.0, 5.1, true)),         // dot has higher precedence now
        '^' | '√' => Some((4.0, 3.9, false)),  // power remains the same
        _ => None,
    }
}

/// Determines how tightly unary ops bind.
///
/// Returns the binding power for prefix operators.
///
/// Higher values mean the operator binds tighter to its operand.
pub fn prefix_binding_power(op: char) -> Option<f32> {
    match op {
        '-' | '+' => Some(20.0),
        '!' => Some(20.0),  // logical NOT
        // root is a unary prefix
        '√' => Some(20.0),
        _ => None,
    }
} 
