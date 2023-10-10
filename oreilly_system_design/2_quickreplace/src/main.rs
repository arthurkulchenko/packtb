mod parseargs;
// use std::env;

use crate::parseargs::*;

fn main() {
    let args = parse_args();
    println!("{:?}", args);
}
