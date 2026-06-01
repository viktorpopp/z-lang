use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Input source file
    pub input: String,
}
