use quicli::prelude::*;
use structopt::StructOpt;

const SHORTENER_SERVICE_URL: &str = "127.0.0.1:3002";

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(logn = "url", short = "u")]
    url: String,
    #[structopt(flatten)]
    verbosity: Verbosity
}

fn main() -> CliResult {
    let args = Cli::from_args();
    println!("Shortening: {}", args.url);
    let client = reqwest::Client::new();
    let mut response = client.post(&format!("http://{}/shorten", SHORTENER_SERVICE_URL)).body(args.url).send()?;
    let shortened_url: String = response.text().unwrap();
    println!("Shortened: {}", shortened_url);
    Ok(())
}
