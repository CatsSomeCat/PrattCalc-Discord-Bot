use ppaaeedb::core::{evaluate, execute, SymbolTable};
use std::error::Error;

//----------------------------------------------------------------------
// Logical Operators Tests
//----------------------------------------------------------------------

/// Tests basic AND, OR operations
#[test]
fn test_basic_logical_operators() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    // Test AND operator
    assert_eq!(evaluate("1 && 1", &mut context)?, 1.0); // true AND true = true
    assert_eq!(evaluate("1 && 0", &mut context)?, 0.0); // true AND false = false
    assert_eq!(evaluate("0 && 1", &mut context)?, 0.0); // false AND true = false
    assert_eq!(evaluate("0 && 0", &mut context)?, 0.0); // false AND false = false
    
    // Test OR operator
    assert_eq!(evaluate("1 || 1", &mut context)?, 1.0); // true OR true = true
    assert_eq!(evaluate("1 || 0", &mut context)?, 1.0); // true OR false = true
    assert_eq!(evaluate("0 || 1", &mut context)?, 1.0); // false OR true = true
    assert_eq!(evaluate("0 || 0", &mut context)?, 0.0); // false OR false = false
    
    Ok(())
}

/// Tests XOR, XNOR operations
#[test]
fn test_xor_operations() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    // Test XOR operator
    assert_eq!(evaluate("1 ^^ 1", &mut context)?, 0.0); // true XOR true = false
    assert_eq!(evaluate("1 ^^ 0", &mut context)?, 1.0); // true XOR false = true
    assert_eq!(evaluate("0 ^^ 1", &mut context)?, 1.0); // false XOR true = true
    assert_eq!(evaluate("0 ^^ 0", &mut context)?, 0.0); // false XOR false = false
    
    // Test XNOR operator
    assert_eq!(evaluate("1 !^ 1", &mut context)?, 1.0); // true XNOR true = true
    assert_eq!(evaluate("1 !^ 0", &mut context)?, 0.0); // true XNOR false = false
    assert_eq!(evaluate("0 !^ 1", &mut context)?, 0.0); // false XNOR true = false
    assert_eq!(evaluate("0 !^ 0", &mut context)?, 1.0); // false XNOR false = true
    
    Ok(())
}

/// Tests NAND, NOR operations
#[test]
fn test_nand_nor_operations() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    // Test NAND operator
    assert_eq!(evaluate("1 !& 1", &mut context)?, 0.0); // true NAND true = false
    assert_eq!(evaluate("1 !& 0", &mut context)?, 1.0); // true NAND false = true
    assert_eq!(evaluate("0 !& 1", &mut context)?, 1.0); // false NAND true = true
    assert_eq!(evaluate("0 !& 0", &mut context)?, 1.0); // false NAND false = true
    
    // Test NOR operator
    assert_eq!(evaluate("1 !| 1", &mut context)?, 0.0); // true NOR true = false
    assert_eq!(evaluate("1 !| 0", &mut context)?, 0.0); // true NOR false = false
    assert_eq!(evaluate("0 !| 1", &mut context)?, 0.0); // false NOR true = false
    assert_eq!(evaluate("0 !| 0", &mut context)?, 1.0); // false NOR false = true
    
    Ok(())
}

/// Tests NOT operation and logical operator precedence
#[test]
fn test_not_and_precedence() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    // Test NOT operator
    assert_eq!(evaluate("!1", &mut context)?, 0.0); // NOT true = false
    assert_eq!(evaluate("!0", &mut context)?, 1.0); // NOT false = true
    
    // Test precedence
    assert_eq!(evaluate("!0 && 1", &mut context)?, 1.0); // (NOT false) AND true = true
    assert_eq!(evaluate("!(0 && 1)", &mut context)?, 1.0); // NOT (false AND true) = true
    assert_eq!(evaluate("1 || 0 && 1", &mut context)?, 1.0); // true OR (false AND true) = true
    
    Ok(())
}

/// Tests using true and false keywords
#[test]
fn test_true_false_keywords() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    // Test basic true/false
    assert_eq!(evaluate("true", &mut context)?, 1.0);
    assert_eq!(evaluate("false", &mut context)?, 0.0);
    
    // Test with operators
    assert_eq!(evaluate("true && false", &mut context)?, 0.0);
    assert_eq!(evaluate("true || false", &mut context)?, 1.0);
    assert_eq!(evaluate("!true", &mut context)?, 0.0);
    assert_eq!(evaluate("!false", &mut context)?, 1.0);
    
    // Test in expressions
    assert_eq!(evaluate("true && (5 > 3)", &mut context)?, 1.0);
    assert_eq!(evaluate("false || (5 < 3)", &mut context)?, 0.0);
    
    Ok(())
}

/// Tests logical operators in program flow
#[test]
fn test_logical_ops_in_program_flow() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    // Test AND in if statement
    let code = r#"
        let x = 0;
        if true && (5 > 3) {
            x = 1;
        } else {
            x = 2;
        }
    "#;
    execute(code, &mut context)?;
    assert_eq!(context.get("x"), Some(&1.0));
    
    // Test OR in if statement
    let code = r#"
        let x = 0;
        if false || (5 > 3) {
            x = 1;
        } else {
            x = 2;
        }
    "#;
    execute(code, &mut context)?;
    assert_eq!(context.get("x"), Some(&1.0));
    
    // Test complex logical expressions
    let code = r#"
        let x = 0;
        if (true && !false) || (false && true) {
            x = 1;
        } else {
            x = 2;
        }
    "#;
    execute(code, &mut context)?;
    assert_eq!(context.get("x"), Some(&1.0));
    
    Ok(())
}
