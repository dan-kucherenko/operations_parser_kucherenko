#[cfg(test)]
mod tests {
    use super::*;
    use operations_parser_kucherenko::{ArithmeticParser, Rule};
    use pest::Parser;

    #[test]
    fn test_number() {
        let input = "42";
        let parsed = ArithmeticParser::parse(Rule::number, input).expect("Failed to parse number");
        assert_eq!(parsed.as_str(), input);
    }

    #[test]
    fn test_unary_op_positive() {
        let input = "+5";
        let parsed =
            ArithmeticParser::parse(Rule::unary, input).expect("Failed to parse unary operation");
        assert_eq!(parsed.as_str(), input);
    }

    #[test]
    fn test_unary_op_negative() {
        let input = "-5";
        let parsed =
            ArithmeticParser::parse(Rule::unary, input).expect("Failed to parse unary operation");
        assert_eq!(parsed.as_str(), input);
    }

    #[test]
    fn test_primary_number() {
        let input = "5.5";
        let parsed =
            ArithmeticParser::parse(Rule::primary, input).expect("Failed to parse primary number");
        assert_eq!(parsed.as_str(), input);
    }

    #[test]
    fn test_primary_function_call() {
        let input = "sqrt(16)";
        let parsed = ArithmeticParser::parse(Rule::primary, input)
            .expect("Failed to parse primary function call");
        assert_eq!(parsed.as_str(), input);
    }

    #[test]
    fn test_add_op() {
        let input = "3 + 2";
        let parsed = ArithmeticParser::parse(Rule::sum, input).expect("Failed to parse addition");
        assert_eq!(parsed.as_str(), input);
    }

    #[test]
    fn test_subtract_op() {
        let input = "5 - 3";
        let parsed =
            ArithmeticParser::parse(Rule::sum, input).expect("Failed to parse subtraction");
        assert_eq!(parsed.as_str(), input);
    }

    #[test]
    fn test_mul_op() {
        let input = "4 * 2";
        let parsed =
            ArithmeticParser::parse(Rule::product, input).expect("Failed to parse multiplication");
        assert_eq!(parsed.as_str(), input);
    }

    #[test]
    fn test_div_op() {
        let input = "10 / 5";
        let parsed =
            ArithmeticParser::parse(Rule::product, input).expect("Failed to parse division");
        assert_eq!(parsed.as_str(), input);
    }

    #[test]
    fn test_power_op() {
        let input = "2 ^ 3";
        let parsed =
            ArithmeticParser::parse(Rule::power, input).expect("Failed to parse exponentiation");
        assert_eq!(parsed.as_str(), input);
    }

    #[test]
    fn test_function_call() {
        let input = "sin(3.14)";
        let parsed = ArithmeticParser::parse(Rule::function_call, input)
            .expect("Failed to parse function call");
        print!("{}", parsed.as_str());
        assert_eq!(parsed.as_str(), input);
    }

    #[test]
    fn test_expression_with_parentheses() {
        let input = "(3 + 5) * 2";
        let parsed = ArithmeticParser::parse(Rule::expression, input)
            .expect("Failed to parse expression with parentheses");
        assert_eq!(parsed.as_str(), input);
    }

    #[test]
    fn test_expression_nested_parentheses() {
        let input = "(3 + (2 * 4))";
        let parsed = ArithmeticParser::parse(Rule::expression, input)
            .expect("Failed to parse expression with nested parentheses");
        assert_eq!(parsed.as_str(), input);
    }

    #[test]
    fn test_expression_with_all_operations() {
        let input = "3 + 5 * 2 - 4 / 2 ^ 2";
        let parsed = ArithmeticParser::parse(Rule::expression, input)
            .expect("Failed to parse expression with all operations");
        assert_eq!(parsed.as_str(), input);
    }
}
