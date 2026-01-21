use std::env;
use text_colorizer::*;

#[derive(Debug)]
#[allow(dead_code)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

fn print_help() {
    eprintln!(
        "{} - replace a string with a new string",
        "Find and Replace".green()
    );
    eprintln!("Usage: <target string> <replacement string> <INPUT FILE> <OUTPUT FILE>"); //Print output goes to std err.
}

pub fn run() {
    let args: Vec<String> = env::args().skip(1).collect(); //Skipping first one, because the first one is the actual initial input that starts the program.
    if args.len() != 4 {
        print_help();
        eprintln!(
            "{} - Wrong number of arguments given. Expected 4, got {}",
            "Error".red().bold(),
            args.len()
        );
        std::process::exit(1);
    };
}
