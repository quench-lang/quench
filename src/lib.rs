use anyhow::{anyhow, Result};
use std::{fs::File, io::Read, path::PathBuf};
use structopt::StructOpt;

mod parser;

#[derive(Debug, StructOpt)]
#[structopt(about)]
struct Opt {
    /// Source file to run as a script
    file: Option<PathBuf>,

    /// Arguments to pass to the script
    args: Vec<String>,
}

pub fn main() -> Result<()> {
    let opt = Opt::from_args();
    let path = opt.file.ok_or(anyhow!(
        "Quench REPL not yet implemented. Please give a filename."
    ))?;
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut parser = parser::parser();
    // parser::parser guarantees we've set a language, and we haven't set timeout or cancellation
    let tree = parser.parse(contents, None).unwrap();
    println!("{}", tree.root_node().to_sexp());

    Ok(())
}
