use text_colorizer::Colorize;
mod parseargs;
// use std::env;
use std::fs;
use regex::Regex;

use crate::parseargs::*;

fn main() {
    let args = parse_args();
    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to read from file '{}' : {:?}", "Error:".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace '{}' with '{}' : {:?}", "Error:".red().bold(), args.target, args.replacement, e);
            std::process::exit(1);
        }
    };
    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} filed to write to file '{}': {:?}", "Error".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    }
}

fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}
