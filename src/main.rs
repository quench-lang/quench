use anyhow::Result;
use quench::db::{self, QueryGroup};
use std::{fs::File, io::Read, path::PathBuf};
use structopt::StructOpt;
use url::Url;

#[derive(Debug, StructOpt)]
#[structopt(about)]
struct Opt {
    /// Source file to run as a script
    file: PathBuf,

    /// Arguments to pass to the script
    args: Vec<String>,
}

fn main() -> Result<()> {
    let ast = {
        let opt = Opt::from_args();
        let uri = Url::from_file_path(opt.file.canonicalize()?).unwrap();
        let mut db = db::Database::default();
        db.open_document(uri.clone(), {
            let mut file = File::open(opt.file)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            contents
        })?;
        db.ast(uri).unwrap()
    };
    println!("{}", ast.0.root_node().to_sexp());
    Ok(())
}
