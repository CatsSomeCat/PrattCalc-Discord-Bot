#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::Expression;

    /// Parses a single numeric atom.
    #[test]
    fn parses_single_atom() {
        let exprs = Expression::parse_from_str("1");
        assert_eq!(exprs[0].to_string(), "1");
    }

    /// Operator precedence: addition and multiplication.
    #[test]
    fn parses_addition_and_multiplication() {
        let exprs = Expression::parse_from_str("1 + 2 * 3");
        assert_eq!(exprs[0].to_string(), "(+ 1 (* 2 3))");
    }

    /// Chain of multiplications.
    #[test]
    fn parses_multiple_multiplications() {
        let exprs = Expression::parse_from_str("a * 2 * b");
        assert_eq!(exprs[0].to_string(), "(* (* a 2) b)");
    }

    /// Mixed operations with precedence.
    #[test]
    fn parses_mixed_operations() {
        let exprs = Expression::parse_from_str("a + b * 2 * c + a / 4");
        assert_eq!(exprs[0].to_string(), "(+ (+ a (* (* b 2) c)) (/ a 4))");
    }

    /// Complex mixed operations.
    #[test]
    fn parses_complex_expression() {
        let exprs = Expression::parse_from_str("2 + b * 5 - 3 / 5 + 5 - 3");
        assert_eq!(exprs[0].to_string(), "(- (+ (- (+ 2 (* b 5)) (/ 3 5)) 5) 3)");
    }

    /// Parentheses change precedence.
    #[test]
    fn parses_parenthesized_expression() {
        let exprs = Expression::parse_from_str("(2 + b) * 5");
        assert_eq!(exprs[0].to_string(), "(* (+ 2 b) 5)");
    }

    /// Redundant parentheses don't affect structure.
    #[test]
    fn parses_nested_parentheses() {
        let exprs = Expression::parse_from_str("(((a)))");
        assert_eq!(exprs[0].to_string(), "a");
    }

    /// Mixed nested operations.
    #[test]
    fn parses_expression_with_nested_parentheses_and_mul_div() {
        let exprs = Expression::parse_from_str("a + b * 2 * (c + a) / 4");
        assert_eq!(exprs[0].to_string(), "(+ a (/ (* (* b 2) (+ c a)) 4))");
    }

    /// Power operator has higher precedence.
    #[test]
    fn parses_expression_with_power() {
        let exprs = Expression::parse_from_str("a + b * c ^ 4");
        assert_eq!(exprs[0].to_string(), "(+ a (* b (^ c 4)))");
    }

    /// Root operator without parentheses.
    #[test]
    fn parses_expression_with_root_operator() {
        let exprs = Expression::parse_from_str("a + 2 √ 4 * b");
        assert_eq!(exprs[0].to_string(), "(+ a (* (√ 2 4) b))");
    }

    /// Root operator with parentheses.
    #[test]
    fn parses_expression_with_root_and_grouped_multiplication() {
        let exprs = Expression::parse_from_str("a + 2 √ (4 * b)");
        assert_eq!(exprs[0].to_string(), "(+ a (√ 2 (* 4 b)))");
    }

    /// Power is right-associative.
    #[test]
    fn parses_nested_power_right_associative() {
        let exprs = Expression::parse_from_str("a ^ b ^ 2");
        assert_eq!(exprs[0].to_string(), "(^ a (^ b 2))");
    }

    /// Dot operator: nested access.
    #[test]
    fn parses_chained_dot_operator() {
        let exprs = Expression::parse_from_str("a.b.c.d");
        assert_eq!(exprs[0].to_string(), "(. (. (. a b) c) d)");
    }

    /// Assignment and pipeline evaluation.
    #[test]
    fn evaluates_variable_assignment_pipeline() {
        let mut context = HashMap::new();
        let result = Expression::evaluate_pipeline("x = 5 | x * 2", &mut context);
        assert_eq!(result, 10.0);
    }

    /// Augmented assignment.
    #[test]
    fn evaluates_augmented_assignment() {
        let mut context = HashMap::new();
        let result = Expression::evaluate_pipeline("x = 3 | x += 4", &mut context);
        assert_eq!(result, 7.0);
    }

    /// Pipeline with multiple assignments and usage.
    #[test]
    fn evaluates_multiple_assignments() {
        let mut context = HashMap::new();
        let result = Expression::evaluate_pipeline("a = 2 | b = 3 | c = a * b | c + 1", &mut context);
        assert_eq!(result, 7.0);
    }

    /// Evaluation with parentheses and operations.
    #[test]
    fn evaluates_expression_with_parentheses() {
        let mut context = HashMap::new();
        let result = Expression::evaluate_pipeline("a = 2 | b = 3 | (a + b) * 2", &mut context);
        assert_eq!(result, 10.0);
    }

    /// Edge case: deeply nested right-associative power.
    #[test]
    fn parses_deeply_nested_power() {
        let exprs = Expression::parse_from_str("2 ^ 3 ^ 2");
        assert_eq!(exprs[0].to_string(), "(^ 2 (^ 3 2))");
    }

    /// Edge case: expression with multiple dot accesses and power.
    #[test]
    fn parses_dot_and_power_combination() {
        let exprs = Expression::parse_from_str("a.b ^ 2.c");
        assert_eq!(exprs[0].to_string(), "(^ (. a b) (. 2 c))");
    }

    /// Edge case: redundant root.
    #[test]
    fn parses_nested_roots() {
        let exprs = Expression::parse_from_str("√ 2 √ 9");
        assert_eq!(exprs[0].to_string(), "(√ 2 (√ 9))");
    }

    /// Evaluation of right-associative power.
    #[test]
    fn evaluates_nested_power_expression() {
        let mut context = HashMap::new();
        let result = Expression::evaluate_pipeline("2 ^ 3 ^ 2", &mut context); // 2^(3^2) = 512
        assert_eq!(result, 512.0);
    }

    /// Evaluation with root operator.
    #[test]
    fn evaluates_root_expression() {
        let mut context = HashMap::new();
        let result = Expression::evaluate_pipeline("2 √ 9", &mut context); // √(9) base 2 = 3
        assert_eq!(result, 3.0);
    }
}
