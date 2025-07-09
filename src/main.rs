// src/main.rs
/*
 * Main executable for TPUTestnetLabDevX
 */

use clap::Parser;
use tputestnetlabdevx::{Result, run};

#[derive(Parser)]
#[command(version, about = "TPUTestnetLabDevX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
