//! Error types for the interpreter.

use std::fmt;
use std::error::Error;

/// Error during parsing of a token stream into an AST.
#[derive(Debug, Clone)]
pub enum ParseError {
    /// Empty token stream.
    EmptyInput,

    /// Unexpected token encountered.
    UnexpectedToken(String),

    /// Unmatched parenthesis.
    UnmatchedParenthesis,

    /// Invalid statement.
    InvalidStatement,

    /// Expected a literal (number or identifier).
    ExpectedLiteral,

    /// Expected an operator.
    ExpectedOperator(String),

    /// Expected a semicolon.
    ExpectedSemicolon,

    /// Expected an identifier.
    ExpectedIdentifier,

    /// Expected a code block.
    ExpectedBlock,

    /// Empty code block.
    EmptyBlock,
    
    /// Invalid number format.
    InvalidNumber(String),
    
    /// Syntax error with message.
    SyntaxError(String),
    
    /// Expected something but found something else.
    Expected {
        /// What was expected
        expected: String,
        /// What was found instead
        found: String,
    },
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::EmptyInput => write!(formatter, "Empty input. Please enter an expression."),
            ParseError::UnexpectedToken(token) => write!(formatter, "Unexpected token: {}", token),
            ParseError::UnmatchedParenthesis => write!(formatter, "Unmatched parenthesis."),
            ParseError::InvalidStatement => write!(formatter, "Invalid statement syntax."),
            ParseError::ExpectedLiteral => write!(formatter, "Expected a literal (number or identifier)."),
            ParseError::ExpectedOperator(context) => write!(formatter, "Expected an operator: {}", context),
            ParseError::ExpectedSemicolon => write!(formatter, "Expected a semicolon."),
            ParseError::ExpectedIdentifier => write!(formatter, "Expected an identifier."),
            ParseError::ExpectedBlock => write!(formatter, "Expected a code block enclosed in curly braces {{}}."),
            ParseError::EmptyBlock => write!(formatter, "Empty code block. A block should contain at least one statement."),
            ParseError::InvalidNumber(msg) => write!(formatter, "Invalid number format: {}", msg),
            ParseError::SyntaxError(msg) => write!(formatter, "Syntax error: {}", msg),
            ParseError::Expected { expected, found } => write!(formatter, "Expected {}, but found {} instead.", expected, found),
        }
    }
}

/// Error during evaluation of an expression.
#[derive(Debug, Clone)]
pub enum EvalError {
    /// Math operation errors
    MathError(MathError),

    /// Variable/symbol related errors
    SymbolError(SymbolError),
    
    /// Control flow errors
    ControlFlowError(ControlFlowError),
}

/// Error during execution of a statement or script.
#[derive(Debug, Clone)]
pub enum ExecutionError {
    /// Invalid statement attempted to execute
    InvalidStatement(String),
    
    /// Type mismatch error
    TypeMismatch {
        /// Expected type
        expected: String,
        /// Actual type
        actual: String,
    },
    
    /// Stack overflow (too much recursion or too complex statement)
    StackOverflow,
    
    /// Maximum execution time exceeded
    TimeoutExceeded,
    
    /// Maximum iterations exceeded
    MaxIterationsExceeded,
    
    /// General execution error with message
    ExecutionFailed(String),
    
    /// Error propagated from expression evaluation
    EvaluationError(EvalError),
}

/// Errors related to mathematical operations
#[derive(Debug, Clone)]
pub enum MathError {
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
    
    /// A domain error for a mathematical function
    DomainError(String),
    
    /// Numerical overflow or underflow
    Overflow,
    
    /// Result is not a number (NaN)
    NotANumber,
}

/// Errors related to variables and symbols
#[derive(Debug, Clone)]
pub enum SymbolError {
    /// A referenced variable was not found in the evaluation context.
    VariableNotFound(String),

    /// An attempt to assign to a variable that hasn't been declared with let.
    UndeclaredVariable(String),

    /// An attempt to modify a constant value.
    ImmutableConstant(String),
    
    /// Redefinition of a constant/variable
    Redefinition(String),
    
    /// Invalid variable or constant name
    InvalidIdentifier(String),
}

/// Errors related to control flow
#[derive(Debug, Clone)]
pub enum ControlFlowError {
    /// A condition evaluated to a non-boolean value.
    NonBooleanCondition,

    /// A break statement was encountered outside a loop.
    BreakOutsideLoop,

    /// A continue statement was encountered outside a loop.
    ContinueOutsideLoop,

    /// A return statement was encountered outside a function.
    ReturnOutsideFunction,
    
    /// Invalid return statement usage
    InvalidReturnStatement(String),
    
    /// Feature is not implemented.
    UnimplementedFeature(String),
    
    /// Function or procedure already defined.
    FunctionOrProcedureAlreadyDefined {
        /// Name of the callable item
        name: String,
        /// Type of the callable item ("function" or "procedure")
        kind: String,
    },
    
    /// Function or procedure not found.
    FunctionOrProcedureNotFound {
        /// Name of the callable item
        name: String,
    },
    
    /// Wrong number of arguments in function call.
    WrongArgumentCount {
        /// Name of the function
        name: String,
        /// Expected number of arguments
        expected: usize,
        /// Actual number of arguments
        got: usize,
    },
}

impl Error for EvalError {}
impl Error for ExecutionError {}
impl Error for MathError {}
impl Error for SymbolError {}
impl Error for ControlFlowError {}

impl fmt::Display for MathError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(formatter, "Division by zero error. Cannot divide by zero."),
            MathError::ModuloByZero => write!(formatter, "Modulo by zero error. Cannot compute modulo with zero divisor."),
            MathError::InvalidExponentiation => write!(formatter, "Invalid exponentiation. Cannot raise a negative number to a fractional power."),
            MathError::UnsupportedOperator(op) => write!(formatter, "Unsupported operator: {}", op),
            MathError::UnsupportedFunction(func) => write!(formatter, "Unsupported function: {}", func),
            MathError::NegativeRoot => write!(formatter, "Cannot compute roots of negative numbers with non-integer degree."),
            MathError::ZerothRoot => write!(formatter, "Cannot compute the zeroth root of a number (mathematically undefined)."),
            MathError::DomainError(msg) => write!(formatter, "Math domain error: {}", msg),
            MathError::Overflow => write!(formatter, "Numerical overflow or underflow occurred."),
            MathError::NotANumber => write!(formatter, "Operation resulted in not-a-number (NaN)."),
        }
    }
}

impl fmt::Display for SymbolError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SymbolError::VariableNotFound(variable) => write!(formatter, "Variable '{}' not found. Make sure it is defined before use.", variable),
            SymbolError::UndeclaredVariable(variable) => write!(formatter, "Undeclared variable: '{}'. Variables must be declared with 'let' before assignment.", variable),
            SymbolError::ImmutableConstant(variable) => write!(formatter, "Cannot modify constant: '{}'. Constants declared with 'const' are immutable.", variable),
            SymbolError::Redefinition(variable) => write!(formatter, "Redefinition of '{}' in the same scope.", variable),
            SymbolError::InvalidIdentifier(name) => write!(formatter, "Invalid identifier name: '{}'.", name),
        }
    }
}

impl fmt::Display for ControlFlowError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ControlFlowError::NonBooleanCondition => write!(formatter, "Condition must evaluate to a boolean value (non-zero for true, zero for false)."),
            ControlFlowError::BreakOutsideLoop => write!(formatter, "Break statement used outside a loop. 'break' can only be used within a 'while' loop."),
            ControlFlowError::ContinueOutsideLoop => write!(formatter, "Continue statement used outside a loop. 'continue' can only be used within a 'while' loop."),
            ControlFlowError::ReturnOutsideFunction => write!(formatter, "Return statement used outside a function or procedure. 'return' can only be used within a function or procedure."),
            ControlFlowError::InvalidReturnStatement(msg) => write!(formatter, "Invalid return statement usage: {}", msg),
            ControlFlowError::UnimplementedFeature(msg) => write!(formatter, "Unimplemented feature: {}", msg),
            ControlFlowError::FunctionOrProcedureAlreadyDefined { name, kind } => write!(formatter, "{} '{}' already defined in the same scope.", kind, name),
            ControlFlowError::FunctionOrProcedureNotFound { name } => write!(formatter, "No callable item named '{}' was found. Make sure the function or procedure is defined before calling it.", name),
            ControlFlowError::WrongArgumentCount { name, expected, got } => write!(formatter, "Callable '{}' called with wrong number of arguments. Expected {}, got {}.", name, expected, got),
        }
    }
}

impl fmt::Display for EvalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EvalError::MathError(error) => write!(formatter, "{}", error),
            EvalError::SymbolError(error) => write!(formatter, "{}", error),
            EvalError::ControlFlowError(error) => write!(formatter, "{}", error),
        }
    }
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::InvalidStatement(msg) => write!(formatter, "Invalid statement: {}", msg),
            ExecutionError::TypeMismatch { expected, actual } => {
                write!(formatter, "Type mismatch: expected {}, found {}", expected, actual)
            },
            ExecutionError::StackOverflow => write!(formatter, "Stack overflow: execution too deeply nested"),
            ExecutionError::TimeoutExceeded => write!(formatter, "Execution timeout exceeded"),
            ExecutionError::MaxIterationsExceeded => write!(formatter, "Maximum iterations exceeded"),
            ExecutionError::ExecutionFailed(msg) => write!(formatter, "Execution failed: {}", msg),
            ExecutionError::EvaluationError(err) => write!(formatter, "Evaluation error: {}", err),
        }
    }
}

impl From<MathError> for EvalError {
    fn from(error: MathError) -> Self {
        EvalError::MathError(error)
    }
}

impl From<SymbolError> for EvalError {
    fn from(error: SymbolError) -> Self {
        EvalError::SymbolError(error)
    }
}

impl From<ControlFlowError> for EvalError {
    fn from(error: ControlFlowError) -> Self {
        EvalError::ControlFlowError(error)
    }
}

impl From<EvalError> for ExecutionError {
    fn from(error: EvalError) -> Self {
        ExecutionError::EvaluationError(error)
    }
}

/// Wrapper error type that can contain any interpreter error.
#[derive(Debug, Clone)]
pub enum InterpreterError {
    /// A parsing error occurred.
    Parse(ParseError),

    /// An evaluation error occurred.
    Eval(EvalError),
    
    /// An execution error occurred.
    Exec(ExecutionError),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpreterError::Parse(error) => write!(formatter, "Parse error: {}", error),
            InterpreterError::Eval(error) => write!(formatter, "Evaluation error: {}", error),
            InterpreterError::Exec(error) => write!(formatter, "Execution error: {}", error),
        }
    }
}

impl Error for InterpreterError {}

impl From<ParseError> for InterpreterError {
    fn from(error: ParseError) -> Self {
        InterpreterError::Parse(error)
    }
}

impl From<EvalError> for InterpreterError {
    fn from(error: EvalError) -> Self {
        InterpreterError::Eval(error)
    }
}

impl From<ExecutionError> for InterpreterError {
    fn from(error: ExecutionError) -> Self {
        InterpreterError::Exec(error)
    }
}
