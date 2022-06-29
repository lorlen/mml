#![forbid(unsafe_code)]

use std::{error::Error, fs, path::PathBuf};

mod data;
mod parser;

use clap::Parser;
use nom::error::ErrorKind;

/// Minimal Markup Language (MML) - a markup language that transpiles to XML (and more),
/// but is more concise and readable.
#[derive(Parser)]
#[clap(about)]
struct Args {
    /// Path to the MML file, or - to read from stdin
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let content = fs::read_to_string(args.input)?;
    let document = parser::document::<(&str, ErrorKind)>(content.as_str()).unwrap();
    println!("{:?}", document);
    Ok(())
}
