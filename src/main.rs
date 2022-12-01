use std::error::Error;
use std::fs;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(
        value_name = "URL",
        required = true,
        help = "URLs to retrieve, e.g. 'https://google.com'"
    )]
    urls: Vec<String>,
}

fn fetch(url: &str) -> Result<(), Box<dyn Error>> {
    let bytes = reqwest::blocking::get(url)?.bytes()?;
    let url_last_name = url
        .split('/')
        .filter(|s| !s.is_empty())
        .last()
        .ok_or("URL cannot be broken down into a filename")?;
    let filename = format!("{}.html", url_last_name);
    fs::write(&filename, bytes)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    for url in args.urls {
        fetch(&url)?;
    }

    Ok(())
}
