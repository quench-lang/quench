use lspower::lsp;
use quench::db::{self, QueryGroup};
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

    AST:

        (source_file (comment) (call function: (identifier) arguments: (arguments (string))))

    No diagnostics.

As you can see, Quench can parse your program, but can't run it yet. Stay tuned!
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

    let ast = db.ast(uri.clone()).unwrap();
    println!("AST:");
    println!();
    println!("    {}", ast.0.root_node().to_sexp());

    println!();

    let diagnostics = db.diagnostics(uri);
    if diagnostics.is_empty() {
        println!("No diagnostics.");
    } else {
        println!("Diagnostics:");
        println!();
        for lsp::Diagnostic {
            severity,
            range: lsp::Range { start, end },
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
                    println!("    {} {:?}: {}", loc, severity, message);
                }
                None => {
                    println!("    {} {}", loc, message);
                }
            }
        }
    }

    Ok(())
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
        Opt::Lsp => Ok(quench::lsp::main()),
        Opt::Run { file, args } => run(file, &args),
    }
}
