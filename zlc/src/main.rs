use anyhow::{Context, Result};
use clap::Parser;
use cli::Cli;
use lalrpop_util::lalrpop_mod;
use std::fs::read_to_string;

mod ast;
mod cli;

lalrpop_mod!(pub grammar);

fn main() -> Result<()> {
    let cli = Cli::parse();

    let source = read_to_string(&cli.input)
        .with_context(|| format!("Failed to read input file: {}", &cli.input))?;

    let ast = grammar::PackageParser::new()
        .parse(source.as_str())
        .unwrap();

    println!("{:#?}", ast);

    Ok(())
}
