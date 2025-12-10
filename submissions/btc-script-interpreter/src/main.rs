mod stack;
mod opcodes;
mod interpreter;
mod transaction;
mod cli;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    cli::run(cli)
}
