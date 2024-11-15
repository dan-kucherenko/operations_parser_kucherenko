#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use operations_parser::{ArithmeticParser, Rule};
    use pest::Parser;

    #[test]
    fn test_number() -> Result<()> {
        let input = "42";
        let parsed = ArithmeticParser::parse(Rule::number, input)
            .map_err(|e| anyhow::anyhow!("Failed to parse number: {:?}", e))?;
        assert_eq!(parsed.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_unary_op_positive() -> Result<()> {
        let input = "+5";
        let parsed = ArithmeticParser::parse(Rule::unary, input)
            .map_err(|e| anyhow::anyhow!("Failed to parse unary operation: {:?}", e))?;
        assert_eq!(parsed.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_unary_op_negative() -> Result<()> {
        let input = "-5";
        let parsed = ArithmeticParser::parse(Rule::unary, input)
            .map_err(|e| anyhow::anyhow!("Failed to parse unary operation: {:?}", e))?;
        assert_eq!(parsed.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_primary_number() -> Result<()> {
        let input = "5.5";
        let parsed = ArithmeticParser::parse(Rule::primary, input)
            .map_err(|e| anyhow::anyhow!("Failed to parse primary number: {:?}", e))?;
        assert_eq!(parsed.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_primary_function_call() -> Result<()> {
        let input = "sqrt(16)";
        let parsed = ArithmeticParser::parse(Rule::primary, input)
            .map_err(|e| anyhow::anyhow!("Failed to parse primary function call: {:?}", e))?;
        assert_eq!(parsed.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_add_op() -> Result<()> {
        let input = "3 + 2";
        let parsed = ArithmeticParser::parse(Rule::sum, input)
            .map_err(|e| anyhow::anyhow!("Failed to parse addition: {:?}", e))?;
        assert_eq!(parsed.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_subtract_op() -> Result<()> {
        let input = "5 - 3";
        let parsed = ArithmeticParser::parse(Rule::sum, input)
            .map_err(|e| anyhow::anyhow!("Failed to parse subtraction: {:?}", e))?;
        assert_eq!(parsed.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_mul_op() -> Result<()> {
        let input = "4 * 2";
        let parsed = ArithmeticParser::parse(Rule::product, input)
            .map_err(|e| anyhow::anyhow!("Failed to parse multiplication: {:?}", e))?;
        assert_eq!(parsed.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_div_op() -> Result<()> {
        let input = "10 / 5";
        let parsed = ArithmeticParser::parse(Rule::product, input)
            .map_err(|e| anyhow::anyhow!("Failed to parse division: {:?}", e))?;
        assert_eq!(parsed.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_power_op() -> Result<()> {
        let input = "2 ^ 3";
        let parsed = ArithmeticParser::parse(Rule::power, input)
            .map_err(|e| anyhow::anyhow!("Failed to parse exponentiation: {:?}", e))?;
        assert_eq!(parsed.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_function_call() -> Result<()> {
        let input = "sin(3.14)";
        let parsed = ArithmeticParser::parse(Rule::function_call, input)
            .map_err(|e| anyhow::anyhow!("Failed to parse function call: {:?}", e))?;
        assert_eq!(parsed.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_expression_with_parentheses() -> Result<()> {
        let input = "(3 + 5) * 2";
        let parsed = ArithmeticParser::parse(Rule::expression, input)
            .map_err(|e| anyhow::anyhow!("Failed to parse expression with parentheses: {:?}", e))?;
        assert_eq!(parsed.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_expression_nested_parentheses() -> Result<()> {
        let input = "(3 + (2 * 4))";
        let parsed = ArithmeticParser::parse(Rule::expression, input)
            .map_err(|e| anyhow::anyhow!("Failed to parse expression with nested parentheses: {:?}", e))?;
        assert_eq!(parsed.as_str(), input);
        Ok(())
    }

    #[test]
    fn test_expression_with_all_operations() -> Result<()> {
        let input = "3 + 5 * 2 - 4 / 2 ^ 2";
        let parsed = ArithmeticParser::parse(Rule::expression, input)
            .map_err(|e| anyhow::anyhow!("Failed to parse expression with all operations: {:?}", e))?;
        assert_eq!(parsed.as_str(), input);
        Ok(())
    }
}
