use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ArithmeticParser;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Failed to parse the arithmetic expression")]
    PestError(#[from] pest::error::Error<Rule>),
}

pub fn parse_expression(input: &str) -> Result<(), ParseError> {
    let pairs = ArithmeticParser::parse(Rule::expression, input)?;

    print_parse_tree(pairs, 0);
    Ok(())
}

fn print_parse_tree(pairs: pest::iterators::Pairs<Rule>, depth: usize) {
    for pair in pairs {
        for _ in 0..depth {
            print!("  ");
        }

        println!("{:?}: {:?}", pair.as_rule(), pair.as_str());

        print_parse_tree(pair.into_inner(), depth + 1);
    }
}
