mod codegen;
mod db;
mod lsp;
mod parser;
mod text;

use crate::{codegen::Codegen, db::QueryGroup};
use std::{env, path::PathBuf};
use structopt::StructOpt;
use url::Url;

const ABOUT: &str = r#"
Here is an example Quench program:

    #!/usr/bin/env quench
    print("Hello, world!");

Save the above contents as hello.qn and run this command:

    quench run hello.qn

Or on a Unix-like system, you could instead run these two commands:

    chmod +x hello.qn
    ./hello.qn

Either way, you should see this output:

    console.log("Hello, world!");

As you can see, Quench compiled your program into JavaScript.
"#;

#[derive(StructOpt)]
#[structopt(about = ABOUT)]
enum Opt {
    /// Starts the language server
    Lsp,
    /// Runs a script
    Run {
        /// Source file to run as a script
        file: PathBuf,

        /// Arguments to pass to the script
        args: Vec<String>,
    },
}

fn run(file: PathBuf, _args: &[String]) -> anyhow::Result<()> {
    let uri = Url::from_file_path(file.canonicalize()?).unwrap();
    let mut db = db::Database::default();
    db.open_document(uri.clone(), slurp::read_all_to_string(file)?)?;

    let diagnostics = db.diagnostics(uri.clone());
    if diagnostics.is_empty() {
        match db
            .compile(uri)
            .and_then(|compiled| Codegen::new().gen(compiled))
        {
            Some(code) => {
                print!("{}", code);
                Ok(())
            }
            None => Err(anyhow::anyhow!("Failed to compile.")),
        }
    } else {
        for lspower::lsp::Diagnostic {
            severity,
            range: lspower::lsp::Range { start, end },
            message,
            ..
        } in diagnostics
        {
            let loc = format!(
                "{}:{} to {}:{}",
                start.line, start.character, end.line, end.character
            );
            match severity {
                Some(severity) => {
                    eprintln!("{} {:?}: {}", loc, severity, message);
                }
                None => {
                    eprintln!("{} {}", loc, message);
                }
            }
        }
        Err(anyhow::anyhow!("Failed to parse."))
    }
}

fn main() -> anyhow::Result<()> {
    // handle shebangs
    if let Some((file, args)) = env::args().skip(1).collect::<Vec<_>>().split_first() {
        // prevent collision with possible future subcommands
        if file.contains("/") {
            return run(PathBuf::from(file), args);
        }
    }
    match Opt::from_args() {
        Opt::Lsp => Ok(lsp::main()),
        Opt::Run { file, args } => run(file, &args),
    }
}
