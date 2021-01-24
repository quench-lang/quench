use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about)]
struct Opt {
    /// Source file to run as a script
    #[structopt(parse(from_os_str))]
    file: Option<PathBuf>,

    /// Arguments to pass to the script
    arg: Vec<String>,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:#?}", opt);
}
