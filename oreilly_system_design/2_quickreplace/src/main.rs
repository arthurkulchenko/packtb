use text_colorizer::Colorize;
mod parseargs;
// use std::env;
use std::fs;

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
    match fs::write(&args.output, &data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{} filed to write to file '{}': {:?}", "Error".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    }
}
