# math_operations_parser_kucherenko

**Math Operations Parser** is a Rust-based library that parses and evaluates arithmetic expressions.

## Features

- Supports basic arithmetic operators: `+`, `-`, `*`, `/`, and `^`
- Handles function calls like `sqrt(16)` and `sin(pi)`
- Can parse complex, nested expressions like `3 + sqrt(16) * (2^3)`

## Technical Overview

This parser uses the Pest grammar and parses expressions in multiple stages:

1. **Expression**: The top-level rule.
2. **Sum**: Handles addition and subtraction.
3. **Product**: Handles multiplication and division.
4. **Power**: Handles exponentiation.
5. **Function Call**: Parses functions with expressions as arguments.

## Usage

To use the parser, run the following command in the terminal:

```sh
cargo run -- parse <file>
```

To get available commands, just type in the commang:

```sh
cargo run -- help
```

For parsing an expression right from the command run:

```sh
cargo run -- "3 + sqrt(16)"
```

To get credits use:

```sh
cargo run -- credits
```

## Grammar Structure

The parser uses a PEG-based grammar defined in `grammar.pest`. Below are the rules that govern expression parsing.

- **expression**: Parses the entire arithmetic expression, starting with `sum`.
- **sum**: Handles addition and subtraction.
- **product**: Handles multiplication and division.
- **power**: Handles exponentiation with `^`.
- **unary**: Handles unary operators (`+` and `-`).
- **primary**: Recognizes numbers, function calls, and grouped expressions (parentheses).
- **function_call**: Matches function names followed by an argument in parentheses.
- **ident**: Matches alphabetic characters used for function names or constants.

### Full Grammar

```pest
WHITESPACE = _{ " " | "\t" }
expression = _{ sum }
sum = { product ~ (add_op ~ product)* }
product = { power ~ (mul_op ~ power)* }
power = { unary ~ (exp_op ~ unary)* }
unary = { unary_op? ~ primary }
primary = { number | function_call | "(" ~ expression ~ ")" }
number = @{ "-"? ~ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? }
add_op = { "+" | "-" }
mul_op = { "*" | "/" }
exp_op = { "^" }
unary_op = { "+" | "-" }
function_call = { ident ~ "(" ~ expression ~ ")" }
ident = @{ ASCII_ALPHA+ }
yaml
Copy code
```
