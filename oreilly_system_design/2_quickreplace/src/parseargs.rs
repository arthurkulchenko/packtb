use std::env;
use text_colorizer::*;

#[derive(Debug)]
pub struct Arguments {
    pub target: String,
    pub replacement: String,
    pub filename: String,
    pub output: String,
}

pub fn print_usage() {
    eprintln!("\n{} - change occurrences of one string into another", "quickreplace".green());
    eprintln!("Usage: quickreplace <target> <replacement> <input> <output>");
}

pub fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 4 {
        print_usage();
        eprintln!("{} expected 4 arguments, got {}", "Error:".red().bold(), args.len());
        std::process::exit(1);
    }

    Arguments {
        target: args[0].clone(),
        replacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}
