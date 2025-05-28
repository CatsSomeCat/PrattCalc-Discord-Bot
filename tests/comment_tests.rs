use ppaaeedb::core::{evaluate, SymbolTable};
use std::error::Error;

//----------------------------------------------------------------------
// Comment Support Tests
//----------------------------------------------------------------------

/// Tests line comments in expressions.
#[test]
fn test_line_comments() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let input = "1 + 2 // This is a comment\n + 3";
    let result = evaluate(input, &mut context)?;
    assert_eq!(result, 6.0);
    Ok(())
}

/// Tests block comments in expressions.
#[test]
fn test_block_comments() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let input = "1 + /* This is a block comment */ 2";
    let result = evaluate(input, &mut context)?;
    assert_eq!(result, 3.0);
    Ok(())
}

/// Tests complex expressions with block comments containing symbols.
#[test]
fn test_complex_comment_expression() -> Result<(), Box<dyn Error>> {
    // Since nested block comments aren't supported, we'll test with a simpler but more complex expression
    let mut context = SymbolTable::<f32>::new();
    let input = "1 + /* Block comment with symbols: +, -, *, / */ 2 * 3";
    let result = evaluate(input, &mut context)?;
    assert_eq!(result, 7.0);
    Ok(())
}

/// Tests expressions with mixed line and block comments.
#[test]
fn test_mixed_comments() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let input = "1 + /* Block comment */\n2 // Line comment\n + 3";
    let result = evaluate(input, &mut context)?;
    assert_eq!(result, 6.0);
    Ok(())
}

/// Tests expressions with comments at the end of the line.
#[test]
fn test_comment_at_end() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let input = "1 + 2 // End comment";
    let result = evaluate(input, &mut context)?;
    assert_eq!(result, 3.0);
    Ok(())
}
