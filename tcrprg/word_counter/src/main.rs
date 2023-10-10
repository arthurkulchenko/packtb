use std::fmt::Display;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Debug)]
struct WordCounter(HashMap<String, u64>);

impl WordCounter {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn increment(&mut self, word: &str) {
        let key = word.to_string();
        let count = self.0.entry(key).or_insert(0);
        *count += 1;
    }

    // TODO: implement Display trait instead of method
    fn display(self) {
        for (key, value) in self.0.iter() {
            println!("{} - {}", value, key);
        }
    }
}

impl Display for WordCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (key, value) in self.0.iter() {
            write!(f, "\n{} - {}", value, key);
        }
        Ok(())
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let filename = &arguments[1];
    println!("Processing filename: {filename}");
    let file = File::open(filename).expect("Could not open file");
    let reader = BufReader::new(file);
    let mut word_counter = WordCounter::new();

    for line in reader.lines() {
        let line = line.expect("Could not read line");
        let words = line.split(" ");
        for word in words {
            if word == "" {
                continue;
            } else {
                word_counter.increment(word);
            }
        }
    }
    // word_counter.display();
    println!("{}", word_counter);
}
