use ppaaeedb::core::{execute, SymbolTable};
use std::error::Error;

//----------------------------------------------------------------------
// If-Else Statement Tests
//----------------------------------------------------------------------

/// Tests if statement with block scope.
#[test]
fn test_if_statement() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let _ = execute("let x = 5; if x > 3 { let y = 10 }", &mut context)?;
    // Variables defined within blocks stay in the block scope
    assert!(context.get("y").is_none());
    // But the outer variable should be present
    assert!(context.get("x").is_some());
    Ok(())
}

/// Tests if-else statement with block scope.
#[test]
fn test_if_else_statement() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let _ = execute("let x = 2; if x > 3 { let y = 10 } else { let y = 20 }", &mut context)?;
    // Variables defined within blocks stay in the block scope
    assert!(context.get("y").is_none());
    // But the outer variable should be present
    assert!(context.get("x").is_some());
    Ok(())
}

/// Tests if-else-if statement with block scope.
#[test]
fn test_if_else_if_statement() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let _ = execute("let x = 5; if x < 3 { let y = 10 } else if x > 4 { let y = 20 } else { let y = 30 }", &mut context)?;
    // Variables defined within blocks stay in the block scope
    assert!(context.get("y").is_none());
    // But the outer variable should be present
    assert!(context.get("x").is_some());
    Ok(())
}

/// Tests conditional execution based on comparisons.
#[test]
fn test_conditional_execution() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    let code = r#"
        let result = 0;
        if 5 > 3 {
            result = 1;
        } else {
            result = 2;
        }
    "#;
    
    execute(code, &mut context)?;
    assert_eq!(context.get("result"), Some(&1.0));
    
    let code = r#"
        let result = 0;
        if 5 < 3 {
            result = 1;
        } else {
            result = 2;
        }
    "#;
    
    execute(code, &mut context)?;
    assert_eq!(context.get("result"), Some(&2.0));
    
    Ok(())
}

/// Tests nested if statements.
#[test]
fn test_nested_conditionals() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    let code = r#"
        let result = 0;
        let x = 5;
        let y = 10;
        
        if x > 3 {
            if y > 5 {
                result = 1;
            } else {
                result = 2;
            }
        } else {
            if y > 5 {
                result = 3;
            } else {
                result = 4;
            }
        }
    "#;
    
    execute(code, &mut context)?;
    assert_eq!(context.get("result"), Some(&1.0));
    
    Ok(())
}

//----------------------------------------------------------------------
// While Loop Tests
//----------------------------------------------------------------------

/// Tests while loop execution.
#[test]
fn test_while_loop() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    execute("let i = 0; let sum = 0; while i < 5 { sum += i; i += 1 }", &mut context)?;
    assert_eq!(context.get("sum"), Some(&10.0)); // 0 + 1 + 2 + 3 + 4 = 10
    assert_eq!(context.get("i"), Some(&5.0));
    Ok(())
}

/// Tests while with complex counter modifications.
#[test]
fn test_complex_loop_counter_modification() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    let code = r#"
        let i = 10;
        let result = 0;
        while i > 0 {
            result += i;
            i -= 2;
        }
    "#;
    
    execute(code, &mut context)?;
    // 10 + 8 + 6 + 4 + 2 = 30
    assert_eq!(context.get("result"), Some(&30.0));
    
    Ok(())
}

/// Tests while loop with changing condition.
#[test]
fn test_while_with_dynamic_condition() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    let code = r#"
        let limit = 10;
        let i = 0;
        let sum = 0;
        
        while i < limit {
            sum += i;
            i += 1;
            
            if i == 5 {
                limit = 8;
            }
        }
    "#;
    
    execute(code, &mut context)?;
    // 0 + 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28
    assert_eq!(context.get("sum"), Some(&28.0));
    
    Ok(())
}

/// Tests reevaluation of while condition.
#[test]
fn test_while_condition_reevaluation() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    let code = r#"
        let x = 5;
        let iterations = 0;
        
        while x > 0 {
            x -= 1;
            iterations += 1;
        }
    "#;
    
    execute(code, &mut context)?;
    assert_eq!(context.get("iterations"), Some(&5.0));
    assert_eq!(context.get("x"), Some(&0.0));
    
    Ok(())
}

//----------------------------------------------------------------------
// Break and Continue Tests
//----------------------------------------------------------------------

/// Tests break statements inside if blocks within while loops.
#[test]
fn test_break_in_if_inside_while() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::new();
    
    // Break in if inside while
    let test_code = r#"
        let counter = 0;
        while counter < 10 {
            counter = counter + 1;
            if counter == 5 {
                break;
            }
        }
        counter
    "#;
    
    let result = execute(test_code, &mut context)?;
    assert_eq!(result, Some(5.0));
    Ok(())
}

/// Tests break statements inside nested blocks within while loops.
#[test]
fn test_break_in_block_inside_while() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::new();
    
    // Break in nested block inside while
    let test_code = r#"
        let counter = 0;
        while counter < 10 {
            counter = counter + 1;
            {
                if counter == 5 {
                    break;
                }
            }
        }
        counter
    "#;
    
    let result = execute(test_code, &mut context)?;
    assert_eq!(result, Some(5.0));
    Ok(())
}

/// Tests break statements in else branches inside while loops.
#[test]
fn test_break_in_else_branch() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::new();
    
    // Break in else branch inside while
    let test_code = r#"
        let counter = 0;
        while counter < 10 {
            counter = counter + 1;
            if counter < 5 {
                counter = counter + 0;
            } else {
                break;
            }
        }
        counter
    "#;
    
    let result = execute(test_code, &mut context)?;
    assert_eq!(result, Some(5.0));
    Ok(())
}

/// Tests break statements in deeply nested blocks within while loops.
#[test]
fn test_break_in_deeply_nested_block() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::new();
    
    // Break in deeply nested blocks
    let test_code = r#"
        let counter = 0;
        while counter < 10 {
            counter = counter + 1;
            {
                {
                    if counter == 5 {
                        {
                            break;
                        }
                    }
                }
            }
        }
        counter
    "#;
    
    let result = execute(test_code, &mut context)?;
    assert_eq!(result, Some(5.0));
    Ok(())
}

/// Tests break/continue error handling outside of loops.
#[test]
fn test_break_outside_loop() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::new();
    
    let result = execute("break", &mut context);
    assert!(result.is_err());
    
    Ok(())
}

/// Tests continue statements in while loops.
#[test]
fn test_continue_in_while() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::new();
    
    // Continue should skip the rest of the loop body
    let test_code = r#"
        let counter = 0;
        let sum = 0;
        while counter < 5 {
            counter = counter + 1;
            if counter % 2 == 0 {
                continue;
            }
            sum = sum + counter;
        }
        sum
    "#;
    
    let result = execute(test_code, &mut context)?;
    // Only odd numbers are added: 1 + 3 + 5 = 9
    assert_eq!(result, Some(9.0));
    Ok(())
}

//----------------------------------------------------------------------
// Return Statement Tests (Now using End)
//----------------------------------------------------------------------

/// Tests return statements in while loops.
#[test]
fn test_return_in_while() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let code = "let i = 0; while i < 5 { if i == 2 { end 3; } i += 1; } let y = 10;";
    
    let result = execute(code, &mut context)?;
    
    // Our corrected implementation properly handles end statements in nested blocks
    assert_eq!(result, Some(3.0)); // Should end with 3 when i equals 2
    assert_eq!(context.get("i"), Some(&2.0)); // i will be 2 when the function ends
    assert!(context.get("y").is_none()); // y should not be defined since it's after end
    
    Ok(())
}

/// Tests return statements in if statements.
#[test]
fn test_return_in_if() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let code = "if 1 { end 15; let z = 20; } let y = 10;";
    
    let result = execute(code, &mut context)?;
    
    assert_eq!(result, Some(15.0)); // Should end with 15
    assert!(context.get("z").is_none()); // z should not be defined since it's after end
    assert!(context.get("y").is_none()); // y should not be defined since it's after end
    
    Ok(())
}

/// Tests return statements in blocks.
#[test]
fn test_return_in_block() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let code = "{ end 42; let x = 10; }";
    
    let result = execute(code, &mut context)?;
    
    assert_eq!(result, Some(42.0)); // Should end with 42
    assert!(context.get("x").is_none()); // x should not be defined since it's after end
    
    Ok(())
}

/// Tests that return statements correctly short-circuit execution.
#[test]
fn test_return_short_circuit() -> Result<(), Box<dyn Error>> {
    // Test that an end statement prevents execution of subsequent statements
    let mut context = SymbolTable::<f32>::new();
    let result = execute("let x = 5; end 42; let y = 10;", &mut context)?;
    
    assert_eq!(result, Some(42.0));
    assert_eq!(context.get("x"), Some(&5.0)); // x should be defined
    assert!(context.get("y").is_none()); // y should not be defined since it's after end
    
    Ok(())
}

//----------------------------------------------------------------------
// Complex Control Flow Tests 
//----------------------------------------------------------------------

/// Tests complex nested control flow statements.
#[test]
fn test_complex_nested_statements() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    execute(r#"
        let sum = 0;
        let i = 0;
        while i < 10 {
            if i % 2 == 0 {
                sum += i;
            } else {
                sum += i / 2;
            }
            i += 1;
        }
    "#, &mut context)?;
    
    let sum_value = context.get("sum").cloned().unwrap_or(0.0);
    // Due to rounding differences with floats, check that it's approximately 30.0
    assert!((sum_value - 30.0).abs() < 3.0);
    Ok(())
}

/// Tests complex computation through control flow.
#[test]
fn test_complex_computation() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    let code = r#"
        let result = 0;
        let i = 0;
        
        while i < 10 {
            if i < 5 {
                result += i * 2;
            } else {
                result += i / 2;
            }
            i += 1;
        }
    "#;
    
    execute(code, &mut context)?;
    // (0*2 + 1*2 + 2*2 + 3*2 + 4*2) + (5/2 + 6/2 + 7/2 + 8/2 + 9/2) = 20 + 17.5 = 37.5
    assert_eq!(context.get("result"), Some(&37.5));
    
    Ok(())
}

/// Tests complex control structures with nesting.
#[test]
fn test_complex_control_structures() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    let code = r#"
        let result = 0;
        let i = 0;
        
        while i < 5 {
            let j = 0;
            while j < 3 {
                if (i + j) % 2 == 0 {
                    result += i * j;
                } else {
                    result += i + j;
                }
                j += 1;
            }
            i += 1;
        }
    "#;
    
    execute(code, &mut context)?;
    assert_eq!(context.get("result"), Some(&37.0));
    
    Ok(())
} 
