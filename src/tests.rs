#[cfg(test)]
mod parsing_tests {
    use crate::Expression;

    /// Parses a single numeric atom.
    #[test]
    fn parses_single_atom() -> Result<(), Box<dyn std::error::Error>> {
        let exprs = Expression::parse_from_str("1")?;
        assert_eq!(exprs[0].to_string(), "1");
        Ok(())
    }

    /// Redundant parentheses don't affect structure.
    #[test]
    fn parses_nested_parentheses() -> Result<(), Box<dyn std::error::Error>> {
        let exprs = Expression::parse_from_str("(((a)))")?;
        assert_eq!(exprs[0].to_string(), "a");
        Ok(())
    }

    /// Dot operator: nested access.
    #[test]
    fn parses_chained_dot_operator() -> Result<(), Box<dyn std::error::Error>> {
        let exprs = Expression::parse_from_str("a.b.c.d")?;
        assert_eq!(exprs[0].to_string(), "(. (. (. a b) c) d)");
        Ok(())
    }
}

#[cfg(test)]
mod operator_precedence_tests {
    use crate::Expression;

    /// Operator precedence: addition and multiplication.
    #[test]
    fn parses_addition_and_multiplication() -> Result<(), Box<dyn std::error::Error>> {
        let exprs = Expression::parse_from_str("1 + 2 * 3")?;
        assert_eq!(exprs[0].to_string(), "(+ 1 (* 2 3))");
        Ok(())
    }

    /// Chain of multiplications.
    #[test]
    fn parses_multiple_multiplications() -> Result<(), Box<dyn std::error::Error>> {
        let exprs = Expression::parse_from_str("a * 2 * b")?;
        assert_eq!(exprs[0].to_string(), "(* (* a 2) b)");
        Ok(())
    }

    /// Mixed operations with precedence.
    #[test]
    fn parses_mixed_operations() -> Result<(), Box<dyn std::error::Error>> {
        let exprs = Expression::parse_from_str("a + b * 2 * c + a / 4")?;
        assert_eq!(exprs[0].to_string(), "(+ (+ a (* (* b 2) c)) (/ a 4))");
        Ok(())
    }

    /// Parentheses change precedence.
    #[test]
    fn parses_parenthesized_expression() -> Result<(), Box<dyn std::error::Error>> {
        let exprs = Expression::parse_from_str("(2 + b) * 5")?;
        assert_eq!(exprs[0].to_string(), "(* (+ 2 b) 5)");
        Ok(())
    }

    /// Mixed nested operations.
    #[test]
    fn parses_expression_with_nested_parentheses_and_mul_div() -> Result<(), Box<dyn std::error::Error>> {
        let exprs = Expression::parse_from_str("a + b * 2 * (c + a) / 4")?;
        assert_eq!(exprs[0].to_string(), "(+ a (/ (* (* b 2) (+ c a)) 4))");
        Ok(())
    }
}

#[cfg(test)]
mod special_operator_tests {
    use crate::Expression;

    /// Power operator has higher precedence.
    #[test]
    fn parses_expression_with_power() -> Result<(), Box<dyn std::error::Error>> {
        let exprs = Expression::parse_from_str("a + b * c ^ 4")?;
        assert_eq!(exprs[0].to_string(), "(+ a (* b (^ c 4)))");
        Ok(())
    }

    /// Power is right-associative.
    #[test]
    fn parses_nested_power_right_associative() -> Result<(), Box<dyn std::error::Error>> {
        let exprs = Expression::parse_from_str("a ^ b ^ 2")?;
        assert_eq!(exprs[0].to_string(), "(^ a (^ b 2))");
        Ok(())
    }

    /// Root operator without parentheses.
    #[test]
    fn parses_expression_with_root_operator() -> Result<(), Box<dyn std::error::Error>> {
        let exprs = Expression::parse_from_str("a + 2 √ 4 * b")?;
        assert_eq!(exprs[0].to_string(), "(+ a (* (√ 2 4) b))");
        Ok(())
    }

    /// Root operator with parentheses.
    #[test]
    fn parses_expression_with_root_and_grouped_multiplication() -> Result<(), Box<dyn std::error::Error>> {
        let exprs = Expression::parse_from_str("a + 2 √ (4 * b)")?;
        assert_eq!(exprs[0].to_string(), "(+ a (√ 2 (* 4 b)))");
        Ok(())
    }

    /// Edge case: redundant root.
    #[test]
    fn parses_nested_roots() -> Result<(), Box<dyn std::error::Error>> {
        let exprs = Expression::parse_from_str("√ 2 √ 9")?;
        assert_eq!(exprs[0].to_string(), "(√ 2 (√ 9))");
        Ok(())
    }
}

#[cfg(test)]
mod edge_case_tests {
    use crate::Expression;

    /// Complex mixed operations.
    #[test]
    fn parses_complex_expression() -> Result<(), Box<dyn std::error::Error>> {
        let exprs = Expression::parse_from_str("2 + b * 5 - 3 / 5 + 5 - 3")?;
        assert_eq!(exprs[0].to_string(), "(- (+ (- (+ 2 (* b 5)) (/ 3 5)) 5) 3)");
        Ok(())
    }

    /// Edge case: deeply nested right-associative power.
    #[test]
    fn parses_deeply_nested_power() -> Result<(), Box<dyn std::error::Error>> {
        let exprs = Expression::parse_from_str("2 ^ 3 ^ 2")?;
        assert_eq!(exprs[0].to_string(), "(^ 2 (^ 3 2))");
        Ok(())
    }

    /// Edge case: expression with multiple dot accesses and power.
    #[test]
    fn parses_dot_and_power_combination() -> Result<(), Box<dyn std::error::Error>> {
        let exprs = Expression::parse_from_str("a.b ^ 2.c")?;
        assert_eq!(exprs[0].to_string(), "(^ (. a b) (. 2 c))");
        Ok(())
    }
}

#[cfg(test)]
mod evaluation_tests {
    use std::collections::HashMap;
    use crate::Expression;

    /// Evaluation of right-associative power.
    #[test]
    fn evaluates_nested_power_expression() -> Result<(), Box<dyn std::error::Error>> {
        let mut context = HashMap::new();
        let result = Expression::evaluate_sequence("2 ^ 3 ^ 2", &mut context)?;
        assert_eq!(result, 512.0);
        Ok(())
    }

    /// Evaluation with root operator.
    #[test]
    fn evaluates_root_expression() -> Result<(), Box<dyn std::error::Error>> {
        let mut context = HashMap::new();
        let result = Expression::evaluate_sequence("2 √ 9", &mut context)?;
        assert_eq!(result, 3.0);
        Ok(())
    }
}