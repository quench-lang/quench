use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about)]
struct Opt {
    /// Source file to run as a script
    file: Option<PathBuf>,

    /// Arguments to pass to the script
    arg: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
