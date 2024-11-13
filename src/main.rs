use operations_parser_kucherenko::parse_expression;
use anyhow::Result;

fn main() -> Result<()> {
    let input = "3 + sqrt(16) * (2^3)";
    parse_expression(input)?;
    Ok(())
}
