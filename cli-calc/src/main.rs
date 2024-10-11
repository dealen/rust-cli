use calc::Calc;
use clap::Parser;

pub mod calc;

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
    
    if result.input.len() == 0 {
        println!("No input provided");
        return;
    }

    let input = result.input;

    let calculate_result = Calc::calculate(&input);
    println!("Result 2: {}", calculate_result);
}

