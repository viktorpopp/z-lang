use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use scanner::Scanner;
use std::fs::read_to_string;

mod cli;
mod scanner;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let source = read_to_string(&cli.input)
        .with_context(|| format!("Failed to read input file: {}", &cli.input))?;

    let mut scanner = Scanner::new(&source);
    let tokens = scanner.scan()?;

    println!("{:#?}", tokens);

    Ok(())
}
