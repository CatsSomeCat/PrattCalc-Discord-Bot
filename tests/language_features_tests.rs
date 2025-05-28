use ppaaeedb::core::{evaluate, execute, SymbolTable};
use std::error::Error;

//----------------------------------------------------------------------
// Expression and Operators Tests
//----------------------------------------------------------------------

/// Tests evaluation of numeric literals.
#[test]
fn test_eval_numeric_literal() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    assert_eq!(evaluate("42", &mut context)?, 42.0);
    Ok(())
}

/// Tests evaluation of decimal numbers.
#[test]
fn test_eval_decimal_number() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    assert_eq!(evaluate("3.14159", &mut context)?, 3.14159);
    Ok(())
}

/// Tests evaluation of hexadecimal numbers.
#[test]
fn test_eval_hex_number() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    assert_eq!(evaluate("0xFF", &mut context)?, 255.0);
    Ok(())
}

/// Tests evaluation of binary numbers.
#[test]
fn test_eval_binary_number() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    assert_eq!(evaluate("0b1010", &mut context)?, 10.0);
    Ok(())
}

/// Tests basic arithmetic operations.
#[test]
fn test_eval_basic_arithmetic() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    assert_eq!(evaluate("2 + 3", &mut context)?, 5.0);
    assert_eq!(evaluate("5 - 2", &mut context)?, 3.0);
    assert_eq!(evaluate("3 * 4", &mut context)?, 12.0);
    assert_eq!(evaluate("10 / 2", &mut context)?, 5.0);
    Ok(())
}

/// Tests operator precedence.
#[test]
fn test_eval_operator_precedence() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    assert_eq!(evaluate("2 + 3 * 4", &mut context)?, 14.0); // 3 * 4 = 12, then 2 + 12 = 14
    assert_eq!(evaluate("2 * 3 + 4", &mut context)?, 10.0); // 2 * 3 = 6, then 6 + 4 = 10
    assert_eq!(evaluate("(2 + 3) * 4", &mut context)?, 20.0); // 2 + 3 = 5, then 5 * 4 = 20
    Ok(())
}

/// Tests nested expressions with parentheses.
#[test]
fn test_eval_nested_expressions() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    assert_eq!(evaluate("(2 + 3) * (4 - 1)", &mut context)?, 15.0);
    assert_eq!(evaluate("2 * (3 + (4 * 5))", &mut context)?, 46.0);
    Ok(())
}

/// Tests complex expressions with multiple operators.
#[test]
fn test_eval_complex_expressions() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    assert_eq!(evaluate("2 + 3 * 4 - 5 / 5", &mut context)?, 13.0);
    assert_eq!(evaluate("10 - 2 * 3 + 5 / 5", &mut context)?, 5.0);
    Ok(())
}

/// Tests unary minus operator.
#[test]
fn test_eval_unary_minus() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    assert_eq!(evaluate("-5", &mut context)?, -5.0);
    assert_eq!(evaluate("--5", &mut context)?, 5.0); // Double negative
    Ok(())
}

/// Tests modulo operator.
#[test]
fn test_eval_modulo() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    assert_eq!(evaluate("10 % 3", &mut context)?, 1.0);
    assert_eq!(evaluate("17 % 5", &mut context)?, 2.0);
    Ok(())
}

/// Tests power operator.
#[test]
fn test_eval_power() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    assert_eq!(evaluate("2 ^ 3", &mut context)?, 8.0);
    assert_eq!(evaluate("3 ^ 2", &mut context)?, 9.0);
    Ok(())
}

/// Tests root operator.
#[test]
fn test_eval_root() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    // Using sqrt function instead of word root
    assert_eq!(evaluate("sqrt(9)", &mut context)?, 3.0);
    assert_eq!(evaluate("sqrt(16)", &mut context)?, 4.0);
    Ok(())
}

/// Tests comparison operators.
#[test]
fn test_eval_comparisons() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    assert_eq!(evaluate("5 > 3", &mut context)?, 1.0); // true
    assert_eq!(evaluate("5 < 3", &mut context)?, 0.0); // false
    assert_eq!(evaluate("5 == 5", &mut context)?, 1.0); // true
    assert_eq!(evaluate("5 != 5", &mut context)?, 0.0); // false
    assert_eq!(evaluate("5 >= 5", &mut context)?, 1.0); // true
    assert_eq!(evaluate("5 <= 3", &mut context)?, 0.0); // false
    Ok(())
}

/// Tests evaluation of variables.
#[test]
fn test_eval_variable() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    context.set_variable("x".to_string(), 42.0)?;
    assert_eq!(evaluate("x", &mut context)?, 42.0);
    Ok(())
}

/// Tests expressions with variables.
#[test]
fn test_eval_expressions_with_variables() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    context.set_variable("x".to_string(), 5.0)?;
    context.set_variable("y".to_string(), 3.0)?;
    assert_eq!(evaluate("x + y", &mut context)?, 8.0);
    assert_eq!(evaluate("x * y", &mut context)?, 15.0);
    assert_eq!(evaluate("x - y", &mut context)?, 2.0);
    assert_eq!(evaluate("x / y", &mut context)?, 5.0 / 3.0);
    Ok(())
}

//----------------------------------------------------------------------
// Constants and Variable Tests
//----------------------------------------------------------------------

/// Tests const declaration and basic usage.
#[test]
fn test_const_declaration() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let result = execute("const pi = 3.14159", &mut context)?;
    
    assert_eq!(result, Some(3.14159));
    assert_eq!(context.get("pi"), Some(&3.14159));
    assert!(context.is_constant("pi"));
    
    Ok(())
}

/// Tests that constants are immutable.
#[test]
fn test_const_immutability() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    // First declare the constant
    execute("const pi = 3.14159", &mut context)?;
    
    // Now try to change it - this should fail
    let result = execute("pi = 3.14", &mut context);
    assert!(result.is_err());
    
    // Verify the value is unchanged
    assert_eq!(context.get("pi"), Some(&3.14159));
    
    Ok(())
}

/// Tests reassigning the same value to a constant (should work).
#[test]
fn test_const_same_value_reassignment() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::new();
    
    // Test reassigning same value to constant
    let test_code = r#"
        const pi = 3.14159;
        pi = 3.14159;
        pi
    "#;
    
    let result = execute(test_code, &mut context)?;
    assert_eq!(result, Some(3.14159));
    Ok(())
}

/// Tests constants with expression initializers.
#[test]
fn test_const_with_expression_initializer() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    // Constant with an expression initializer
    let result = execute("const x = 2 * 3 + 4", &mut context)?;
    
    assert_eq!(result, Some(10.0));
    assert_eq!(context.get("x"), Some(&10.0));
    assert!(context.is_constant("x"));
    
    Ok(())
}

/// Tests redeclaring constants (should fail).
#[test]
fn test_const_redeclaration() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    // First declaration
    execute("const pi = 3.14159", &mut context)?;
    
    // Try to redeclare - should fail
    let result = execute("const pi = 3.14", &mut context);
    assert!(result.is_err());
    
    // Verify the original value is preserved
    assert_eq!(context.get("pi"), Some(&3.14159));
    
    Ok(())
}

/// Tests using constants in expressions.
#[test]
fn test_using_constants_in_expressions() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    // Define some constants
    execute("const pi = 3.14159; const r = 5", &mut context)?;
    
    // Use them in an expression
    let result = execute("pi * r * r", &mut context)?;
    
    assert_eq!(result, Some(3.14159 * 5.0 * 5.0));
    
    Ok(())
}

/// Tests constants and variables together.
#[test]
fn test_constants_and_variables() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    // Define a constant and a variable
    execute("const pi = 3.14159; let r = 5", &mut context)?;
    
    // Use them together
    let result = execute("pi * r * r", &mut context)?;
    assert_eq!(result, Some(3.14159 * 5.0 * 5.0));
    
    // Try to modify the variable (should succeed)
    execute("r = 7", &mut context)?;
    assert_eq!(context.get("r"), Some(&7.0));
    
    // The computed value should now be different
    let result = execute("pi * r * r", &mut context)?;
    assert_eq!(result, Some(3.14159 * 7.0 * 7.0));
    
    Ok(())
}

/// Tests constants in blocks.
#[test]
fn test_constants_in_blocks() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    // Declare a constant in a block
    execute("{ const local_pi = 3.14159 }", &mut context)?;
    
    // Constant should not be visible outside the block
    assert!(context.get("local_pi").is_none());
    
    // But a constant declared outside should be visible everywhere
    execute("const pi = 3.14159; { pi }", &mut context)?;
    assert_eq!(context.get("pi"), Some(&3.14159));
    
    Ok(())
}

//----------------------------------------------------------------------
// Variable and Assignment Tests
//----------------------------------------------------------------------

/// Tests variable declaration with initialization.
#[test]
fn test_let_statement() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let result = execute("let x = 42", &mut context)?;
    assert_eq!(context.get("x"), Some(&42.0));
    assert_eq!(result, Some(42.0));
    Ok(())
}

/// Tests variable declaration without initialization.
#[test]
fn test_let_without_initializer() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let result = execute("let x", &mut context)?;
    assert_eq!(context.get("x"), Some(&0.0)); // Default value is 0.0
    assert_eq!(result, Some(0.0));
    Ok(())
}

/// Tests variable declaration with complex expression initializer.
#[test]
fn test_let_with_expression_initializer() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let result = execute("let x = 2 * 3 + 4", &mut context)?;
    assert_eq!(context.get("x"), Some(&10.0)); // 2*3 + 4 = 10
    assert_eq!(result, Some(10.0));
    Ok(())
}

/// Tests direct assignment to existing variables.
#[test]
fn test_simple_assignment() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    execute("let x = 5", &mut context)?;
    let result = execute("x = 42", &mut context)?;
    assert_eq!(context.get("x"), Some(&42.0));
    assert_eq!(result, Some(42.0));
    Ok(())
}

/// Tests compound assignment operators.
#[test]
fn test_augmented_assignment() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    execute("let x = 5", &mut context)?;
    
    execute("x += 3", &mut context)?;
    assert_eq!(context.get("x"), Some(&8.0));
    
    execute("x -= 2", &mut context)?;
    assert_eq!(context.get("x"), Some(&6.0));
    
    execute("x *= 3", &mut context)?;
    assert_eq!(context.get("x"), Some(&18.0));
    
    execute("x /= 2", &mut context)?;
    assert_eq!(context.get("x"), Some(&9.0));
    
    execute("x %= 4", &mut context)?;
    assert_eq!(context.get("x"), Some(&1.0));
    
    Ok(())
} 
