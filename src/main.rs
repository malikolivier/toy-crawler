use std::error::Error;
use std::fs;

use chrono::Utc;
use clap::Parser;

mod metadata;

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
    let last_fetch = Utc::now();
    let url_last_name = url
        .split('/')
        .filter(|s| !s.is_empty())
        .last()
        .ok_or("URL cannot be broken down into a filename")?;
    let filename = format!("{}.html", url_last_name);
    let metadata = metadata::get_metadata(&bytes);
    println!("site:\t\t{}", url);
    println!("num_links:\t{}", metadata.num_links);
    println!("num_images:\t{}", metadata.num_images);
    println!("last_fetch:\t{}\n", last_fetch);
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
