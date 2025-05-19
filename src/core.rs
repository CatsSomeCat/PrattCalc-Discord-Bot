// Standard library imports (alphabetical)
use std::{collections::HashMap, fmt, error::Error};

/// Error type for parsing expressions.
#[derive(Debug)]
#[allow(dead_code)]
pub enum ParseError {
    /// An unexpected token was encountered during parsing.
    UnexpectedToken(String),

    /// A closing parenthesis was missing or unmatched.
    UnmatchedParenthesis,

    /// A specific token was expected but not found.
    ExpectedToken(String),

    /// An attempt was made to parse an empty expression.
    EmptyExpression,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken(t) => write!(f, "Unexpected token: {}", t),
            ParseError::UnmatchedParenthesis => write!(f, "Unmatched parenthesis"),
            ParseError::ExpectedToken(t) => write!(f, "Expected token: {}", t),
            ParseError::EmptyExpression => write!(f, "Empty expression"),
        }
    }
}

impl Error for ParseError {}

/// Error type for evaluating expressions.
#[derive(Debug)]
pub enum EvalError {
    /// Division by zero was attempted.
    DivisionByZero,

    /// Modulo by zero was attempted.
    ModuloByZero,

    /// Exponentiation was performed with invalid operands.
    InvalidExponentiation,

    /// An unsupported operator was encountered.
    UnsupportedOperator(char),

    /// A function call was made to an unsupported function.
    UnsupportedFunction(String),

    /// A root operation on a negative number was attempted.
    NegativeRoot,

    /// A zeroth root was attempted (mathematically undefined).
    ZerothRoot,

    /// A referenced variable was not found in the evaluation context.
    VariableNotFound(String),
}

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::DivisionByZero => write!(f, "Division by zero"),
            EvalError::ModuloByZero => write!(f, "Modulo by zero"),
            EvalError::InvalidExponentiation => write!(f, "Invalid exponentiation operation"),
            EvalError::UnsupportedOperator(op) => write!(f, "Unsupported operator: {}", op),
            EvalError::UnsupportedFunction(func) => write!(f, "Unsupported function: {}", func),
            EvalError::NegativeRoot => write!(f, "Attempted to take a root of a negative number"),
            EvalError::ZerothRoot => write!(f, "Attempted to compute the zeroth root"),
            EvalError::VariableNotFound(var) => write!(f, "Variable not found: {}", var),
        }
    }
}

impl Error for EvalError {}

/// A token in the expression language.
///
/// Includes literals, single-character operators, augmented assignments, and end-of-input marker.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    /// A literal: number (decimal, hex, binary), identifier, or function name marker.
    Literal(String),

    /// A single-character operator, e.g., '+', '-', '^', '√', '.'.
    Operator(char),

    /// An augmented assignment operator, e.g., "+=", "-=", "*=", "/=", "%=".
    AugAssign(String),

    /// End of input marker.
    EndOfInput,
}

/// Tokenizer splits the raw input string into a sequence of tokens.
#[derive(Debug)]
struct Tokenizer {
    token_list: Vec<Token>,
}

impl Tokenizer {
    /// Constructs a tokenizer from raw input, removing whitespace
    /// and recognizing numeric literals (decimal, hex, binary), identifiers, functions, operators,
    /// single-char operators, augmented assignments, and dot-access.
    fn from_input(input: &str) -> Self {
        let mut token_list = Vec::new();
        let mut chars_iter = input.chars().filter(|ch| !ch.is_whitespace()).peekable();

        while let Some(&current_char) = chars_iter.peek() {
            // Number literal parsing, including leading '.' if followed by digit
            if current_char.is_ascii_digit() || (current_char == '.' && chars_iter.clone().nth(1).is_some_and(|ch| ch.is_ascii_digit())) {
                let mut literal_text = String::new();

                // Handle leading '0' for hex/binary as before
                if current_char == '0' {
                    chars_iter.next();
                    literal_text.push('0');
                    if let Some(&next_char) = chars_iter.peek() {
                        match next_char {
                            'x' | 'X' => {
                                literal_text.push(next_char);
                                chars_iter.next();
                                while let Some(&hex_digit) = chars_iter.peek() {
                                    if hex_digit.is_ascii_hexdigit() {
                                        literal_text.push(hex_digit);
                                        chars_iter.next();
                                    } else {
                                        break;
                                    }
                                }
                                token_list.push(Token::Literal(literal_text));
                                continue;
                            }
                            'b' | 'B' => {
                                literal_text.push(next_char);
                                chars_iter.next();
                                while let Some(&bin_digit) = chars_iter.peek() {
                                    if bin_digit == '0' || bin_digit == '1' {
                                        literal_text.push(bin_digit);
                                        chars_iter.next();
                                    } else {
                                        break;
                                    }
                                }
                                token_list.push(Token::Literal(literal_text));
                                continue;
                            }
                            _ => {}
                        }
                    }
                }

                // Handle decimal digits with at most one dot
                let mut dot_encountered = false;
                while let Some(&peek_char) = chars_iter.peek() {
                    if peek_char.is_ascii_digit() {
                        literal_text.push(peek_char);
                        chars_iter.next();
                    } else if peek_char == '.' && !dot_encountered {
                        // Check if this dot is part of a floating-point number or field access
                        if let Some(next_next) = chars_iter.clone().nth(1) {
                            if next_next.is_ascii_digit() {
                                dot_encountered = true;
                                literal_text.push(peek_char);
                                chars_iter.next();
                            } else {
                                // Not a number; it's a dot-access
                                break;
                            }
                        } else {
                            // Trailing dot; treat as operator
                            break;
                        }
                    } else {
                        break;
                    }
                }

                token_list.push(Token::Literal(literal_text));
            }
            // Identifier or function parsing
            else if current_char.is_ascii_alphabetic() {
                let mut identifier = String::new();
                while let Some(&peek_char) = chars_iter.peek() {
                    if peek_char.is_ascii_alphanumeric() || peek_char == '_' {
                        identifier.push(peek_char);
                        chars_iter.next();
                    } else {
                        break;
                    }
                }
                if let Some(&paren_char) = chars_iter.peek() {
                    if paren_char == '(' {
                        token_list.push(Token::Literal(format!("fn:{}", identifier)));
                    } else {
                        token_list.push(Token::Literal(identifier));
                    }
                } else {
                    token_list.push(Token::Literal(identifier));
                }
            }
            // Operator or augmented assignment parsing
            else {
                let first_char = chars_iter.next().unwrap();

                // Check for two-char augmented assignment ops
                if let Some(&second_char) = chars_iter.peek() {
                    let two_char_op = format!("{}{}", first_char, second_char);
                    if matches!(two_char_op.as_str(), "+=" | "-=" | "*=" | "/=" | "%=") {
                        chars_iter.next();
                        token_list.push(Token::AugAssign(two_char_op));
                        continue;
                    }
                }

                // Treat '.' as operator, not literal
                if first_char == '.' {
                    token_list.push(Token::Operator('.'));
                } else {
                    token_list.push(Token::Operator(first_char));
                }
            }
        }

        token_list.reverse(); Tokenizer { token_list }
    }


    /// Consume and return the next token, or `EndOfInput` if no tokens remain.
    fn next_token(&mut self) -> Token {
        self.token_list.pop().unwrap_or(Token::EndOfInput)
    }

    /// Peek at the upcoming token without consuming it.
    fn peek_token(&self) -> &Token {
        self.token_list.last().unwrap_or(&Token::EndOfInput)
    }
}

/// AST node for expressions.
#[derive(Clone)]
pub enum Expression {
    /// A literal string: numeric or variable identifier.
    Literal(String),

    /// An operation: operator character and operand subexpressions.
    Operation(char, Vec<Expression>),

    /// A function call: function name and argument expressions.
    FunctionCall(String, Vec<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, target: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Literal(value) => write!(target, "{}", value),
            Expression::Operation(operator, operands) => {
                write!(target, "({}", operator)?;
                for operand in operands {
                    write!(target, " {}", operand)?;
                }
                write!(target, ")")
            }
            Expression::FunctionCall(name, args) => {
                write!(target, "{}(", name)?;
                for (index, arg_expr) in args.iter().enumerate() {
                    if index > 0 {
                        write!(target, ", ")?;
                    }
                    write!(target, "{}", arg_expr)?;
                }
                write!(target, ")")
            }
        }
    }
}

impl Expression {
    /// Parse a single expression (no pipeline support).
    pub fn parse_single(input: &str) -> Result<Self, ParseError> {
        let mut tokenizer = Tokenizer::from_input(input);
        parse_expression(&mut tokenizer, 0.0)
    }

    /// Parse a pipeline of expressions separated by '|'.
    pub fn parse_from_str(input: &str) -> Result<Vec<Expression>, ParseError> {
        let expressions: Result<Vec<Expression>, ParseError> = input
            // Changed from '|' to ';'
            .split(';')
            .map(str::trim)
            .filter(|segment| !segment.is_empty())
            .map(Self::parse_single)
            .collect();

        expressions
    }

    /// Identify if this is an assignment operation.
    /// 
    /// Returns `(variable_name, rhs_expression)` if so.
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
    pub fn evaluate(&self, context: &HashMap<String, f32>) -> Result<f32, EvalError> {
        match self {
            // Literal: number or variable
            Expression::Literal(text) => {
                // Hexadecimal (0xFF)
                if let Some(hex_digits) = text.strip_prefix("0x") {
                    let value = u32::from_str_radix(hex_digits, 16)
                        .map_err(|_| EvalError::InvalidExponentiation)? as f32;
                    return Ok(value);
                }
                // Binary (0b1010)
                if let Some(bin_digits) = text.strip_prefix("0b") {
                    let value = u32::from_str_radix(bin_digits, 2)
                        .map_err(|_| EvalError::InvalidExponentiation)? as f32;
                    return Ok(value);
                }
                // Decimal literal
                if let Ok(decimal_value) = text.parse::<f32>() {
                    return Ok(decimal_value);
                }
                // Variable lookup
                context.get(text)
                    .cloned()
                    .ok_or_else(|| EvalError::VariableNotFound(text.clone()))
            }

            // === Infix or prefix operation (unary, binary, root) ===
            Expression::Operation(operator, operands) => {
                // Evaluate left operand (always present)
                let left_val = operands[0].evaluate(context)?;
                // Evaluate right operand if binary, otherwise default to 0.0
                let right_val = if operands.len() > 1 {
                    operands[1].evaluate(context)?
                } else {
                    0.0
                };

                match *operator {
                    // Arithmetic
                    '+' => Ok(left_val + right_val),
                    '-' if operands.len() == 1 => Ok(-left_val), // unary minus
                    '-' => Ok(left_val - right_val),
                    '*' => Ok(left_val * right_val),

                    // Division with zero check
                    '/' => {
                        if right_val == 0.0 {
                            Err(EvalError::DivisionByZero)
                        } else {
                            Ok(left_val / right_val)
                        }
                    }

                    // Modulo with zero check
                    '%' => {
                        if right_val == 0.0 {
                            Err(EvalError::ModuloByZero)
                        } else {
                            Ok(left_val % right_val)
                        }
                    }

                    // Exponentiation: check for invalid negative base + fractional exponent
                    '^' => {
                        if left_val < 0.0 && right_val.fract() != 0.0 {
                            Err(EvalError::InvalidExponentiation)
                        } else {
                            Ok(left_val.powf(right_val))
                        }
                    }

                    // Root operation: expects exactly two operands
                    '√' => {
                        if operands.len() != 2 {
                            return Err(EvalError::InvalidExponentiation);
                        }
                        let degree = left_val;
                        let radicand = right_val;
                        if degree == 0.0 {
                            Err(EvalError::ZerothRoot)
                        } else if radicand < 0.0 && (1.0 / degree).fract() != 0.0 {
                            Err(EvalError::NegativeRoot)
                        } else {
                            Ok(radicand.powf(1.0 / degree))
                        }
                    }

                    // Dot-access operator: returns the right-hand side
                    '.' => Ok(right_val),

                    // Unsupported operator
                    other => Err(EvalError::UnsupportedOperator(other)),
                }
            }

            // Function call
            Expression::FunctionCall(name, args) => {
                // Evaluate all arguments first
                let mut evaluated_args = Vec::with_capacity(args.len());
                for arg in args {
                    evaluated_args.push(arg.evaluate(context)?);
                }

                // Dispatch built-in functions
                match name.as_str() {
                    "sin"   => Ok(evaluated_args[0].sin()),
                    "cos"   => Ok(evaluated_args[0].cos()),
                    "tan"   => Ok(evaluated_args[0].tan()),
                    "log"   => Ok(evaluated_args[0].ln()),
                    "sqrt"  => Ok(evaluated_args[0].sqrt()),
                    "abs"   => Ok(evaluated_args[0].abs()),
                    "max"   => Ok(evaluated_args[0].max(evaluated_args[1])),
                    "min"   => Ok(evaluated_args[0].min(evaluated_args[1])),
                    unknown => Err(EvalError::UnsupportedFunction(unknown.to_string())),
                }
            }
        }
    }

    /// Parse and evaluate a sequence of semicolon-separated expressions,
    /// updating the variable context and returning the last computed result.
    #[allow(dead_code)]
    pub fn evaluate_sequence(
        input: &str,
        context: &mut HashMap<String, f32>,
    ) -> Result<f32, Box<dyn std::error::Error>> {
        let mut last_value = 0.0;
        let mut tokenizer = Tokenizer::from_input(input);
        while tokenizer.peek_token() != &Token::EndOfInput {
            let expr = parse_expression(&mut tokenizer, 0.0)?;
            // Assignment detection
            if let Expression::Operation('=', operands) = &expr {
                if let Expression::Literal(var_name) = &operands[0] {
                    let value = operands[1].evaluate(context)?;
                    context.insert(var_name.clone(), value);
                    last_value = value;
                    continue;
                }
            }
            last_value = expr.evaluate(context)?;
        }
        Ok(last_value)
    }
}

/// Defines precedence and associativity.
fn infix_binding_power(op: char) -> Option<(f32, f32, bool)> {
    match op {
        '=' => Some((0.2, 0.1, false)),        // right-associative
        '+' | '-' => Some((1.0, 1.1, true)),   // left-associative
        '*' | '/' | '%' => Some((2.0, 2.1, true)),
        '.' => Some((5.0, 5.1, true)),         // dot has higher precedence now
        '^' | '√' => Some((4.0, 3.9, false)),  // power remains the same
        _ => None,
    }
}

/// Determines how tightly unary ops bind.
fn prefix_binding_power(op: char) -> Option<f32> {
    match op {
        '-' | '+' => Some(20.0),
        // root is a unary prefix
        '√' => Some(20.0),
        _ => None,
    }
}

/// Parses an expression using Pratt parsing (based on operator precedence).
/// 
/// Returns an abstract syntax tree (AST) representing the parsed expression.
fn parse_expression(tokenizer: &mut Tokenizer, min_bp: f32) -> Result<Expression, ParseError> {
    // Parse the left-hand side (LHS) operand or expression
    let mut lhs = match tokenizer.peek_token() {
        // Grouped expression: (expr)
        Token::Operator('(') => {
            // consume '('
            tokenizer.next_token(); let expr = parse_expression(tokenizer, 0.0)?;
            match tokenizer.next_token() {
                Token::Operator(')') => expr,
                _ => return Err(ParseError::UnmatchedParenthesis),
            }
        }

        // Prefix operator or unary/root expression (e.g., -a, √a, a √ b)
        Token::Operator(op) if prefix_binding_power(*op).is_some() => {
            // consume operator
            let prefix_op = *op; tokenizer.next_token();
            let binding_power = prefix_binding_power(prefix_op).unwrap();

            // Parse the operand following the prefix operator
            let first_operand = parse_expression(tokenizer, binding_power)?;
            let mut operands = vec![first_operand];

            // Special case for √ operator that may accept a second operand (e.g., a √ b)
            if prefix_op == '√' && matches!(
                tokenizer.peek_token(),
                Token::Literal(_) | Token::Operator('(') | Token::Operator('√')
            ) {
                operands.push(parse_expression(tokenizer, binding_power)?);
            }

            Expression::Operation(prefix_op, operands)
        }

        // Literal or potential function call (e.g., "fn:sin")
        Token::Literal(_) => {
            if let Token::Literal(lit) = tokenizer.next_token() {
                if lit.starts_with("fn:") {
                    let function_name = lit.trim_start_matches("fn:").to_string();
                    
                    // consume '('
                    tokenizer.next_token(); let mut args = Vec::new();

                    while tokenizer.peek_token() != &Token::Operator(')') {
                        args.push(parse_expression(tokenizer, 0.0)?);
                        if tokenizer.peek_token() == &Token::Operator(',') {
                             // consume ','
                            tokenizer.next_token();
                        } else {
                            break;
                        }
                    }
                    
                    // consume ')'
                    tokenizer.next_token(); Expression::FunctionCall(function_name, args)
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

    // Parse infix and augmented operators
    loop {
        match tokenizer.peek_token() {
            // End of expression or expression group
            Token::EndOfInput | Token::Operator(')') | Token::Operator(',') => break,

            // Infix operators (e.g., +, -, *, /, ^, etc.)
            Token::Operator(op) if infix_binding_power(*op).is_some() => {
                let (left_bp, right_bp, is_left_associative) = infix_binding_power(*op).unwrap();

                // Stop parsing if operator precedence is lower than current context
                if (is_left_associative && left_bp < min_bp)
                    || (!is_left_associative && left_bp <= min_bp)
                {
                    break;
                }
                
                // consume operator
                let operator = *op; tokenizer.next_token();
                let rhs = parse_expression(tokenizer, right_bp)?;
                lhs = Expression::Operation(operator, vec![lhs, rhs]);
            }

            // Augmented assignment (e.g., +=, -=, *=, etc.)
            Token::AugAssign(_) => {
                if let Token::AugAssign(aug_op_str) = tokenizer.next_token() {
                    // Extract actual operator from the augmented assignment (e.g., "+=" -> '+')
                    let base_op = aug_op_str.chars().next().unwrap();
                    let rhs = parse_expression(tokenizer, 0.0)?;

                    // Desugar x += y => x = x + y
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
