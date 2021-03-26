use lspower::lsp;
use quench::db::{self, QueryGroup};
use std::path::PathBuf;
use structopt::StructOpt;
use url::Url;

const ABOUT: &str = r#"
Here is an example Quench program:

    #!/usr/bin/env quench
    print("Hello, world!");

Save the above contents as hello.qn and run these commands:

    chmod +x hello.qn
    ./hello.qn

You should see this output:

    AST:

        (source_file (comment) (call function: (identifier) arguments: (arguments (string))))

    No diagnostics.

As you can see, Quench can parse your program, but can't run it yet. Stay tuned!
"#;

#[derive(Debug, StructOpt)]
#[structopt(about = ABOUT)]
struct Opt {
    /// Source file to run as a script
    file: PathBuf,

    /// Arguments to pass to the script
    args: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let uri = Url::from_file_path(opt.file.canonicalize()?).unwrap();
    let mut db = db::Database::default();
    db.open_document(uri.clone(), slurp::read_all_to_string(opt.file)?)?;

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
