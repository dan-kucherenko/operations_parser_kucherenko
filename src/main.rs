use anyhow::Result;
use operations_parser_kucherenko::parse_expression;
use std::env;
use std::fs;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("parse") => {
            if let Some(file) = args.get(2) {
                parse_file(file)?;
            } else {
                eprintln!("Please specify a file to parse.");
            }
        }
        Some("help") => {
            print_help();
        }
        Some("credits") => {
            print_credits();
        }
        Some(expression) => {
            parse_expression(expression)?;
        }
        None => {
            eprintln!("No arguments provided. Use 'help' for usage.");
        }
    }

    Ok(())
}

/// Parses each line in the specified file
fn parse_file(filename: &str) -> Result<()> {
    let content = fs::read_to_string(filename)
        .map_err(|_| anyhow::anyhow!("Failed to read file: {}", filename))?;

    for line in content.lines() {
        println!("Parsing line: {}", line);
        parse_expression(line)?;
    }
    Ok(())
}

fn print_help() {
    println!("Expression Parser CLI");
    println!("Usage:");
    println!("  parse <file>       Parses expressions from the specified file");
    println!("  <expression>       Parses a single expression directly from the command line");
    println!("  help               Displays this help information");
    println!("  credits            Shows project credits");
}

fn print_credits() {
    println!("Math Operations Parser v0.1.0");
    println!("Developed by Daniil Kucherenko");
}
