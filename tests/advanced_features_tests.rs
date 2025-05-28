use ppaaeedb::core::{evaluate, execute, SymbolTable};
use std::error::Error;

//----------------------------------------------------------------------
// Function Tests
//----------------------------------------------------------------------

/// Tests the random number generator function
#[test]
fn test_random_number_generator() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    // Test rand() with no arguments (should return 0-1)
    let result = evaluate("rand()", &context)?;
    assert!(result >= 0.0 && result <= 1.0);
    
    // Test rand(10) - should return 0-10
    let result = evaluate("rand(10)", &context)?;
    assert!(result >= 0.0 && result <= 10.0);
    
    // Test rand(5, 10) - should return 5-10
    let result = evaluate("rand(5, 10)", &context)?;
    assert!(result >= 5.0 && result <= 10.0);
    
    // Test invalid arguments
    let result = evaluate("rand(10, 5)", &context);
    assert!(result.is_err());
    
    Ok(())
} 

/// Tests the absolute value function.
#[test]
fn test_abs_function() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    assert_eq!(evaluate("abs(5)", &context)?, 5.0);
    assert_eq!(evaluate("abs(-5)", &context)?, 5.0);
    assert_eq!(evaluate("abs(0)", &context)?, 0.0);
    
    Ok(())
}

/// Tests the square root function.
#[test]
fn test_sqrt_function() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    assert_eq!(evaluate("sqrt(4)", &context)?, 2.0);
    assert_eq!(evaluate("sqrt(9)", &context)?, 3.0);
    
    Ok(())
}

/// Tests the sine function.
#[test]
fn test_sin_function() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    // Use approximate comparison for floating point values
    let result = evaluate("sin(0)", &context)?;
    assert!((result - 0.0).abs() < 0.0001);
    
    Ok(())
}

/// Tests the cosine function.
#[test]
fn test_cos_function() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    // Use approximate comparison for floating point values
    let result = evaluate("cos(0)", &context)?;
    assert!((result - 1.0).abs() < 0.0001);
    
    Ok(())
}

/// Tests the tangent function.
#[test]
fn test_tan_function() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    // Use approximate comparison for floating point values
    let result = evaluate("tan(0)", &context)?;
    assert!((result - 0.0).abs() < 0.0001);
    
    Ok(())
}

/// Tests the logarithm function.
#[test]
fn test_log_function() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    // log(e) = 1
    let result = evaluate("log(2.718281)", &context)?;
    assert!((result - 1.0).abs() < 0.01);
    
    Ok(())
}

/// Tests the min function.
#[test]
fn test_min_function() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    assert_eq!(evaluate("min(5, 10)", &context)?, 5.0);
    assert_eq!(evaluate("min(-5, 10)", &context)?, -5.0);
    
    Ok(())
}

/// Tests the max function.
#[test]
fn test_max_function() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    assert_eq!(evaluate("max(5, 10)", &context)?, 10.0);
    assert_eq!(evaluate("max(-5, 10)", &context)?, 10.0);
    
    Ok(())
}

/// Tests functions with expressions as arguments.
#[test]
fn test_functions_with_expressions() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    assert_eq!(evaluate("abs(5 - 10)", &context)?, 5.0);
    assert_eq!(evaluate("sqrt(3 * 3)", &context)?, 3.0);
    
    Ok(())
}

/// Tests using functions with variables.
#[test]
fn test_functions_with_variables() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    execute("let x = 9", &mut context)?;
    assert_eq!(evaluate("sqrt(x)", &context)?, 3.0);
    
    Ok(())
}

/// Tests nested function calls.
#[test]
fn test_nested_functions() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    assert_eq!(evaluate("abs(sqrt(16) - 6)", &context)?, 2.0);
    assert_eq!(evaluate("sqrt(abs(-9))", &context)?, 3.0);
    
    Ok(())
}

/// Tests the cotangent function
#[test]
fn test_cot_function() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    // Test cotangent of π/4 (should be 1.0)
    let result = evaluate("cot(3.14159 / 4)", &context)?;
    assert!((result - 1.0).abs() < 0.01);
    
    // Test that cot(x) = 1/tan(x)
    let tan_result = evaluate("tan(0.5)", &context)?;
    let cot_result = evaluate("cot(0.5)", &context)?;
    
    assert!((cot_result - 1.0/tan_result).abs() < 0.00001);
    
    Ok(())
}

/// Tests the secant function
#[test]
fn test_sec_function() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    // Test secant of 0 (should be 1.0)
    let result = evaluate("sec(0)", &context)?;
    assert!((result - 1.0).abs() < 0.01);
    
    // Test secant of π/3
    let result = evaluate("sec(3.14159 / 3)", &context)?;
    assert!((result - 2.0).abs() < 0.01);
    
    Ok(())
}

/// Tests the cosecant function
#[test]
fn test_csc_function() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    // Test cosecant of π/2 (should be 1.0)
    let result = evaluate("csc(3.14159 / 2)", &context)?;
    assert!((result - 1.0).abs() < 0.01);
    
    // Test cosecant of π/6
    let result = evaluate("csc(3.14159 / 6)", &context)?;
    assert!((result - 2.0).abs() < 0.01);
    
    Ok(())
}

/// Tests the inverse trigonometric functions
#[test]
fn test_inverse_trig_functions() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    // Test inverse sine
    let result = evaluate("asin(0)", &context)?;
    assert!((result - 0.0).abs() < 0.0001);
    
    // Test inverse cosine
    let result = evaluate("acos(1)", &context)?;
    assert!((result - 0.0).abs() < 0.0001);
    
    // Test inverse tangent
    let result = evaluate("atan(0)", &context)?;
    assert!((result - 0.0).abs() < 0.0001);
    
    Ok(())
}

/// Tests the atan2 function
#[test]
fn test_atan2_function() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    // Test atan2 of (0, 1) - should be 0.0
    let result = evaluate("atan2(0, 1)", &context)?;
    assert!((result - 0.0).abs() < 0.0001);
    
    // Test atan2 of (1, 0) - should be π/2
    let result = evaluate("atan2(1, 0)", &context)?;
    assert!((result - 1.5708).abs() < 0.01);
    
    // Test atan2 of (0, -1) - should be π
    let result = evaluate("atan2(0, -1)", &context)?;
    assert!((result - 3.1416).abs() < 0.01);
    
    // Test invalid number of arguments
    let result = evaluate("atan2(1)", &context);
    assert!(result.is_err());
    
    Ok(())
}

//----------------------------------------------------------------------
// Complex Expression Tests
//----------------------------------------------------------------------

/// Tests complex numeric expressions with multiple operators.
#[test]
fn test_complex_numeric_expression() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    // This is a complex expression with multiple operators
    let result = evaluate("(2.5 + 3.75) * 4 - 5 / 2.5 + 10 % 3", &context)?;
    // (2.5 + 3.75) * 4 - 5 / 2.5 + 10 % 3 = 6.25 * 4 - 2 + 1 = 25 - 2 + 1 = 24
    assert!((result - 24.0).abs() < 0.001);
    
    Ok(())
}

/// Tests extremely deeply nested expressions.
#[test]
fn test_deeply_nested_expressions() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    // Deeply nested expression with multiple levels of parentheses
    let result = evaluate("((((1 + 2) * (3 - 1)) / 2) ^ 2) - 1", &context)?;
    // ((((1 + 2) * (3 - 1)) / 2) ^ 2) - 1 = (((3 * 2) / 2) ^ 2) - 1 = ((6 / 2) ^ 2) - 1 = (3 ^ 2) - 1 = 9 - 1 = 8
    assert_eq!(result, 8.0);
    
    Ok(())
}

/// Tests expressions with extreme values.
#[test]
fn test_extreme_values() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    // Very large number
    let result = evaluate("1000000 * 1000000", &context)?;
    assert_eq!(result, 1000000000000.0);
    
    // Very small number
    let result = evaluate("0.0000001 * 10000000", &context)?;
    assert!((result - 1.0).abs() < 0.001);
    
    Ok(())
}

/// Tests logical operators in non-zero conditions.
#[test]
fn test_logical_operators() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    // Test direct values in statements - non-zero is treated as true
    let code = r#"
        let x = 0;
        if 1 { x = 1; } else { x = 2; }
    "#;
    execute(code, &mut context)?;
    assert_eq!(context.get("x"), Some(&1.0));
    
    // Test direct values in statements - zero is treated as false
    let code = r#"
        let x = 0;
        if 0 { x = 1; } else { x = 2; }
    "#;
    execute(code, &mut context)?;
    assert_eq!(context.get("x"), Some(&2.0));
    
    // Test comparison operators
    let code = r#"
        let x = 0;
        if 5 > 3 { x = 1; } else { x = 2; }
    "#;
    execute(code, &mut context)?;
    assert_eq!(context.get("x"), Some(&1.0));
    
    let code = r#"
        let x = 0;
        if 5 < 3 { x = 1; } else { x = 2; }
    "#;
    execute(code, &mut context)?;
    assert_eq!(context.get("x"), Some(&2.0));
    
    Ok(())
}

/// Tests precedence edge cases.
#[test]
fn test_precedence_edge_cases() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    // Testing precedence edge cases
    let result = evaluate("2 + 3 * 4 ^ 2", &context)?;
    assert_eq!(result, 2.0 + 3.0 * 16.0); // 4^2 = 16, 3*16 = 48, 2+48 = 50
    
    let result = evaluate("10 - 2 - 3", &context)?;
    assert_eq!(result, 5.0); // Left to right: (10-2)-3 = 8-3 = 5
    
    Ok(())
}

/// Tests complex expressions with many variables.
#[test]
fn test_many_variables() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    execute("let a = 5; let b = 7; let c = 9", &mut context)?;
    
    assert_eq!(evaluate("a + b + c", &context)?, 21.0);
    assert_eq!(evaluate("a * b - c", &context)?, 26.0);
    
    Ok(())
}

/// Tests combinations of root and power operators.
#[test]
fn test_root_and_power_combinations() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    // Test power then root
    let result = evaluate("sqrt(4 ^ 2)", &context)?;
    assert_eq!(result, 4.0); // √(4²) = √16 = 4
    
    // Test root then power
    let result = evaluate("(sqrt(9)) ^ 2", &context)?;
    assert_eq!(result, 9.0); // (√9)² = 3² = 9
    
    Ok(())
}

/// Tests mixed numeric types in expressions.
#[test]
fn test_mixed_numeric_types() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    // Mix integers and floating point
    let result = evaluate("5 + 3.5", &context)?;
    assert_eq!(result, 8.5);
    
    // Mix hexadecimal, binary, and decimal
    let result = evaluate("0xFF + 0b1010 + 15.5", &context)?;
    assert_eq!(result, 255.0 + 10.0 + 15.5); // 255 + 10 + 15.5 = 280.5
    
    Ok(())
}

//----------------------------------------------------------------------
// Scope Tests
//----------------------------------------------------------------------

/// Tests variable shadowing in blocks.
#[test]
fn test_variable_shadowing() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    execute("let x = 10; { let x = 20; }", &mut context)?;
    
    // After the block, x should still be 10
    assert_eq!(context.get("x"), Some(&10.0));
    
    Ok(())
}

/// Tests constant shadowing.
#[test]
fn test_constant_shadowing() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    // Create separate scopes for the constants
    execute("const x = 10;", &mut context)?;
    execute("{ const y = 20; }", &mut context)?;
    
    // The outer x should be defined, but y should not be accessible
    assert_eq!(context.get("x"), Some(&10.0));
    assert!(context.get("y").is_none());
    
    Ok(())
}

/// Tests accessing outer scope variables from inner scope.
#[test]
fn test_outer_scope_access() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    execute("let x = 10; { let y = x + 5; }", &mut context)?;
    
    assert_eq!(context.get("x"), Some(&10.0));
    // y is not in outer scope
    assert!(context.get("y").is_none());
    
    Ok(())
}

/// Tests modifying outer variables from inner scope.
#[test]
fn test_outer_variable_modification() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    execute("let x = 10; { x = 20; }", &mut context)?;
    
    // The modification should persist
    assert_eq!(context.get("x"), Some(&20.0));
    
    Ok(())
}

/// Tests shadowing combined with reassignment.
#[test]
fn test_shadowing_with_reassignment() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    execute("let x = 10; { let x = 20; x = 30; }", &mut context)?;
    
    // Outer x should be unchanged
    assert_eq!(context.get("x"), Some(&10.0));
    
    Ok(())
}

/// Tests multi-level shadowing.
#[test]
fn test_multi_level_shadowing() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    execute("let x = 10; { let x = 20; { let x = 30; } }", &mut context)?;
    
    // Outer x should be unchanged
    assert_eq!(context.get("x"), Some(&10.0));
    
    Ok(())
}

/// Tests shadowing in control structures.
#[test]
fn test_control_structure_shadowing() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    execute("let x = 10; if 1 { let x = 20; }", &mut context)?;
    
    // Outer x should be unchanged
    assert_eq!(context.get("x"), Some(&10.0));
    
    Ok(())
}

/// Tests complex scope interactions.
#[test]
fn test_complex_scope_interactions() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    let code = r#"
        let a = 10;
        let b = 20;
        
        {
            let a = 30;
            b = b + a; // b = 20 + 30 = 50
            
            {
                let c = 5;
                a = a + c; // a = 30 + 5 = 35 (inner a)
            }
            
            // c is out of scope
            // a is still 35 (inner a)
        }
        
        // a is still 10 (outer a)
        // b is now 50
    "#;
    
    execute(code, &mut context)?;
    
    assert_eq!(context.get("a"), Some(&10.0));
    assert_eq!(context.get("b"), Some(&50.0));
    assert!(context.get("c").is_none());
    
    Ok(())
}

/// Tests predefined mathematical constants
#[test]
fn test_predefined_constants() -> Result<(), Box<dyn Error>> {
    let context = SymbolTable::<f32>::new();
    
    // Test PI constant
    let result = evaluate("PI", &context)?;
    assert!((result - std::f32::consts::PI).abs() < 0.0001);
    
    // Test TAU constant (2π)
    let result = evaluate("TAU", &context)?;
    assert!((result - (std::f32::consts::PI * 2.0)).abs() < 0.0001);
    
    // Test E constant
    let result = evaluate("E", &context)?;
    assert!((result - std::f32::consts::E).abs() < 0.0001);
    
    // Test PHI constant (golden ratio)
    let result = evaluate("PHI", &context)?;
    assert!((result - 1.618033988749895).abs() < 0.0001);
    
    // Test SQRT2 constant
    let result = evaluate("SQRT2", &context)?;
    assert!((result - std::f32::consts::SQRT_2).abs() < 0.0001);
    
    // Test INFINITY constant
    let result = evaluate("INFINITY", &context)?;
    assert!(result.is_infinite() && result.is_sign_positive());
    
    // Test constants in expressions
    let result = evaluate("2 * PI", &context)?;
    assert!((result - (std::f32::consts::PI * 2.0)).abs() < 0.0001);
    
    Ok(())
} 

/// Tests that global constants can be accessed even with an empty context.
#[test]
fn test_global_constants_access() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    // Constants should be available even with an empty context
    let result = evaluate("PI", &mut context)?;
    assert!((result - std::f32::consts::PI).abs() < 0.0001);
    
    let result = evaluate("E", &mut context)?;
    assert!((result - std::f32::consts::E).abs() < 0.0001);
    
    Ok(())
}

/// Tests that global constants are immutable.
#[test]
fn test_global_constants_immutable() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    
    // Try to modify a constant (should fail)
    let result = execute("PI = 3", &mut context);
    assert!(result.is_err());
    
    // But we can use them in expressions
    let result = evaluate("2 * PI", &mut context)?;
    assert!((result - 2.0 * std::f32::consts::PI).abs() < 0.0001);
    
    Ok(())
}

//----------------------------------------------------------------------
// End Keyword Tests
//----------------------------------------------------------------------

/// Tests basic usage of the end keyword
#[test]
fn test_end_keyword_basic() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let code = "let x = 5; end x + 10;";
    
    let result = execute(code, &mut context)?;
    
    assert_eq!(result, Some(15.0)); // Should end with x + 10
    assert_eq!(context.get("x"), Some(&5.0)); // x should be defined
    
    Ok(())
}

/// Tests that end keyword short-circuits execution
#[test]
fn test_end_short_circuit() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let code = "let x = 5; end x * 3; let y = 10;";
    
    let result = execute(code, &mut context)?;
    
    assert_eq!(result, Some(15.0)); // Should end with x * 3
    assert_eq!(context.get("x"), Some(&5.0)); // x should be defined
    assert!(context.get("y").is_none()); // y should not be defined since it's after end
    
    Ok(())
}

/// Tests the end keyword in nested blocks
#[test]
fn test_end_in_nested_blocks() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let code = "let x = 5; { let y = 10; end x + y; let z = 15; }; let w = 20;";
    
    let result = execute(code, &mut context)?;
    
    assert_eq!(result, Some(15.0)); // Should end with x + y
    assert_eq!(context.get("x"), Some(&5.0)); // x should be defined
    assert!(context.get("z").is_none()); // z should not be defined since it's after end
    assert!(context.get("w").is_none()); // w should not be defined since it's after end
    
    Ok(())
}

/// Tests the end keyword in conditional blocks
#[test]
fn test_end_in_conditionals() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let code = "
        let x = 5; 
        if x > 3 { 
            let y = 10; 
            end x * y; 
            let z = 15; 
        } else { 
            end x; 
        }; 
        let w = 20;
    ";
    
    let result = execute(code, &mut context)?;
    
    assert_eq!(result, Some(50.0)); // Should end with x * y (5 * 10)
    assert_eq!(context.get("x"), Some(&5.0)); // x should be defined
    assert!(context.get("z").is_none()); // z should not be defined since it's after end
    assert!(context.get("w").is_none()); // w should not be defined since it's after end
    
    Ok(())
}

/// Tests the end keyword in loops
#[test]
fn test_end_in_loops() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let code = "
        let sum = 0;
        let i = 0;
        while i < 10 {
            sum += i;
            i += 1;
            if i == 5 {
                // End when i reaches 5
                end sum;
            }
        }
        // This should never be reached
        let final_value = 100;
    ";
    
    let result = execute(code, &mut context)?;
    
    assert_eq!(result, Some(10.0)); // Sum of 0 + 1 + 2 + 3 + 4 = 10
    assert_eq!(context.get("i"), Some(&5.0)); // i should be 5 when the program ends
    assert_eq!(context.get("sum"), Some(&10.0)); // sum should be 10
    assert!(context.get("final_value").is_none()); // final_value should not be defined
    
    Ok(())
}

/// Tests that end with no expression returns None
#[test]
fn test_end_without_expression() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let code = "let x = 5; end;";
    
    let result = execute(code, &mut context)?;
    
    assert_eq!(result, None); // end with no expression should return None
    assert_eq!(context.get("x"), Some(&5.0)); // x should still be defined
    
    Ok(())
}

/// Tests that the return keyword now produces an error
#[test]
fn test_return_produces_error() -> Result<(), Box<dyn Error>> {
    let mut context = SymbolTable::<f32>::new();
    let code = "let x = 5; return x;";
    
    let result = execute(code, &mut context);
    
    assert!(result.is_err()); // return should produce an error
    assert!(result.unwrap_err().to_string().contains("Use 'end' instead")); // Error message should mention using 'end'
    
    Ok(())
} 
