use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    script: PathBuf,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args.script);
}
