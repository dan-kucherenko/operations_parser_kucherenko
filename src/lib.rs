use anyhow::{Context, Result};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ArithmeticParser;

pub fn parse_expression(input: &str) -> Result<()> {
    let pairs = ArithmeticParser::parse(Rule::expression, input)
        .context("Failed to parse the arithmetic expression")?;

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
