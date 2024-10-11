#![warn(clippy::all, clippy::pedantic)]
use clap::Parser;
use cli_calc::calc::Calc;

#[derive(Parser)]
struct Cli {
    input: String,
}

fn main() {
    let matches = Cli::try_parse();

    if matches.is_err() {
        println!("No input provided");
        return;
    }

    let result = matches.unwrap();

    if result.input.is_empty() {
        println!("No input provided");
        return;
    }

    let input = result.input;

    let calculate_result = Calc::calculate(&input);
    println!("Result: {calculate_result}", );
}
