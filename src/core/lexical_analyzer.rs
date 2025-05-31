//! Lexical analysis module for tokenizing input.
//! 
//! This module converts raw input text into tokens for the parser.

use std::iter::Peekable;
use std::str::Chars;
use crate::core::error_types::ParseError;

/// A token in the expression language.
///
/// Includes literals, operators, keywords, and structural elements.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// A numeric literal: decimal (123, 3.14), hex (0xFF), binary (0b101).
    Literal(String),

    /// A single-character operator, e.g., '+', '-', '^', '√', '.'.
    Operator(char),

    /// An augmented assignment operator, e.g., "+=", "-=", "*=", "/=", "%=".
    AugAssign(String),

    /// Keywords for control flow and declarations.
    Keyword(String),

    /// End of input marker.
    EndOfInput,
}

/// Tokenizer splits the raw input string into a sequence of tokens.
///
/// The tokenizer performs lexical analysis on the input string, converting
/// it into a stream of tokens that can be processed by the parser.
#[derive(Debug, Clone)]
pub struct Tokenizer {
    /// The list of tokens generated from the input.
    pub(crate) token_list: Vec<Token>,
    
    /// Current position in the token stream.
    position: usize,
}

impl Tokenizer {
    /// Constructs a tokenizer from raw input, performing lexical analysis.
    ///
    /// This method processes the input string and produces a sequence of tokens
    /// by recognizing patterns like numbers, identifiers, operators, etc.
    pub fn from_input(input: &str) -> Self {
        let mut tokenizer = Self {
            token_list: Vec::new(),
            position: 0,
        };
        
        tokenizer.tokenize(input);
        
        tokenizer
    }
    
    /// Tokenizes the input string into a sequence of tokens.
    fn tokenize(&mut self, input: &str) {
        let mut token_list = Vec::with_capacity(input.len() / 2); // Reasonable estimate
        let mut chars_iter = input.chars().peekable();

        // Keywords that the tokenizer should recognize
        const KEYWORDS: [&str; 13] = [
            "if", "else", "while", "break", "continue", 
            "return", "let", "const", "true", "false", "end",
            "fn", "proc"
        ];

        while let Some(&current_char) = chars_iter.peek() {
            // Skip whitespace between tokens
            if current_char.is_whitespace() {
                chars_iter.next();
                continue;
            }

            match current_char {
                // Handle comments
                '/' => {
                    if self.try_parse_comment(&mut chars_iter) {
                        continue;
                    }
                    
                    // If not a comment, treat as division operator
                    chars_iter.next();
                    token_list.push(Token::Operator('/'));
                },
                
                // Handle numeric literals
                '0'..='9' => {
                    let literal = self.parse_number(&mut chars_iter);
                    token_list.push(Token::Literal(literal));
                },
                
                // Handle decimal point starting a number
                '.' => {
                    // Look ahead to see if this is the start of a number
                    let mut lookahead = chars_iter.clone();
                    lookahead.next(); // Skip the '.'
                    
                    if lookahead.next().map_or(false, |c| c.is_ascii_digit()) {
                        let literal = self.parse_number_with_leading_dot(&mut chars_iter);
                        token_list.push(Token::Literal(literal));
                    } else {
                        // Just a dot operator
                        chars_iter.next();
                        token_list.push(Token::Operator('.'));
                    }
                },
                
                // Handle identifiers and keywords
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let text = self.parse_identifier(&mut chars_iter);
                    
                    // Check if it's a keyword
                    if KEYWORDS.contains(&text.as_str()) {
                        // Special handling for boolean literals
                        match text.as_str() {
                            "true" => token_list.push(Token::Literal("1".to_string())),
                            "false" => token_list.push(Token::Literal("0".to_string())),
                            _ => token_list.push(Token::Keyword(text)),
                        }
                    }
                    // Check if it's a function or procedure call (function or procedure name followed by an opening parenthesis)
                    else if let Some(&paren_char) = chars_iter.peek() {
                        if paren_char == '(' {
                            token_list.push(Token::Literal(text));
                        } else {
                            token_list.push(Token::Literal(text));
                        }
                    } else {
                        token_list.push(Token::Literal(text));
                    }
                },
                
                // Single-character punctuation
                '(' | ')' | '{' | '}' | ';' | ',' => {
                    chars_iter.next();
                    token_list.push(Token::Operator(current_char));
                },
                
                // Operators that could be part of augmented assignments
                '+' | '-' | '*' | '%' | '^' => {
                    chars_iter.next();
                    
                    if let Some(&next_char) = chars_iter.peek() {
                        if next_char == '=' {
                            chars_iter.next();
                            token_list.push(Token::AugAssign(format!("{}=", current_char)));
                        } else {
                            token_list.push(Token::Operator(current_char));
                        }
                    } else {
                        token_list.push(Token::Operator(current_char));
                    }
                },
                
                // Comparison operators
                '=' | '<' | '>' | '!' => {
                    chars_iter.next();
                    
                    if let Some(&next_char) = chars_iter.peek() {
                        if next_char == '=' {
                            chars_iter.next();
                            token_list.push(Token::AugAssign(format!("{}=", current_char)));
                        } else {
                            token_list.push(Token::Operator(current_char));
                        }
                    } else {
                        token_list.push(Token::Operator(current_char));
                    }
                },
                
                // Other recognized operators
                '√' => {
                    chars_iter.next();
                    token_list.push(Token::Operator('√'));
                },
                
                // Skip unrecognized characters (could add error reporting here)
                _ => { chars_iter.next(); },
            }
        }
        
        // Always add an end-of-input marker
        token_list.push(Token::EndOfInput);
        self.token_list = token_list;
    }

    /// Attempts to parse a comment. Returns true if a comment was consumed.
    fn try_parse_comment(&self, chars: &mut Peekable<Chars>) -> bool {
        let mut lookahead = chars.clone();
        lookahead.next(); // Skip the '/'
        
        match lookahead.next() {
            // Line comment: //
            Some('/') => {
                chars.next(); // Skip first '/'
                chars.next(); // Skip second '/'
                
                // Skip until end of line or input
                while let Some(&ch) = chars.peek() {
                    if ch == '\n' {
                        chars.next();
                        break;
                    }
                    chars.next();
                }
                true
            },
            
            // Block comment: /* ... */
            Some('*') => {
                chars.next(); // Skip '/'
                chars.next(); // Skip '*'
                
                let mut _found_end = false;
                while let Some(ch) = chars.next() {
                    if ch == '*' && chars.peek() == Some(&'/') {
                        chars.next(); // Skip '/'
                        _found_end = true;
                        break;
                    }
                }
                
                // We disregard unclosed comments for now
                true
            },
            
            // Not a comment
            _ => false,
        }
    }
    
    /// Parses a numeric literal starting with a digit.
    fn parse_number(&self, chars: &mut Peekable<Chars>) -> String {
        let mut number = String::new();
        let first_char = chars.next().unwrap();
        number.push(first_char);
        
        // Handle hexadecimal (0x...) or binary (0b...) literals
        if first_char == '0' {
            if let Some(&next) = chars.peek() {
                match next {
                    'x' | 'X' => {
                        chars.next(); // Consume 'x'
                        number.push('x');
                        
                        // Parse hex digits
                        while let Some(&ch) = chars.peek() {
                            if ch.is_ascii_hexdigit() {
                                number.push(ch);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        return number;
                    },
                    'b' | 'B' => {
                        chars.next(); // Consume 'b'
                        number.push('b');
                        
                        // Parse binary digits
                        while let Some(&ch) = chars.peek() {
                            if ch == '0' || ch == '1' {
                                number.push(ch);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        return number;
                    },
                    _ => {}
                }
            }
        }
        
        // Parse regular decimal digits
        self.parse_decimal_digits(chars, &mut number, false)
    }
    
    /// Parses a numeric literal starting with a decimal point.
    fn parse_number_with_leading_dot(&self, chars: &mut Peekable<Chars>) -> String {
        let mut number = String::new();
        number.push('.');
        chars.next(); // Consume the '.'
        
        self.parse_decimal_digits(chars, &mut number, true)
    }
    
    /// Helps to parse decimal digits and decimal points.
    fn parse_decimal_digits(&self, chars: &mut Peekable<Chars>, number: &mut String, has_dot: bool) -> String {
        let mut dot_encountered = has_dot;
        
        while let Some(&ch) = chars.peek() {
            if ch.is_ascii_digit() {
                number.push(ch);
                chars.next();
            } else if ch == '.' && !dot_encountered {
                // Check if followed by a digit
                let mut lookahead = chars.clone();
                lookahead.next(); // Skip the dot
                
                if lookahead.next().map_or(false, |c| c.is_ascii_digit()) {
                    dot_encountered = true;
                    number.push('.');
                    chars.next();
                } else {
                    // The dot is not part of this number
                    break;
                }
            } else {
                break;
            }
        }
        
        number.clone()
    }
    
    /// Parses an identifier (variable name or function name).
    fn parse_identifier(&self, chars: &mut Peekable<Chars>) -> String {
        let mut identifier = String::new();
        
        while let Some(&ch) = chars.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                identifier.push(ch);
                chars.next();
            } else {
                break;
            }
        }
        
        identifier
    }
    
    /// Gets the next token from the stream and advances the position.
    pub fn next_token(&mut self) -> Token {
        if self.position >= self.token_list.len() {
            Token::EndOfInput
        } else {
            let token = self.token_list[self.position].clone();
            self.position += 1;
            token
        }
    }
    
    /// Looks at the current token without advancing the position.
    pub fn peek_token(&self) -> &Token {
        if self.position >= self.token_list.len() {
            &Token::EndOfInput
        } else {
            &self.token_list[self.position]
        }
    }
    
    /// Checks if the next token is of a specific type.
    pub fn check(&self, expected: &Token) -> bool {
        self.peek_token() == expected
    }
    
    /// Expects the next token to be of a specific type, advancing position if it matches.
    pub fn expect(&mut self, expected: Token) -> Result<Token, ParseError> {
        let token = self.next_token();
        if token == expected {
            Ok(token)
        } else {
            Err(ParseError::Expected { 
                expected: format!("{:?}", expected),
                found: format!("{:?}", token),
            })
        }
    }
    
    /// Resets the tokenizer position back to the beginning.
    pub fn reset(&mut self) {
        self.position = 0;
    }
} 
