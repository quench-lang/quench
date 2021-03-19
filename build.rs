use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    let grammar_file = "grammar.js";
    println!("cargo:rerun-if-changed={}", grammar_file);

    let ts_name = "tree-sitter-quench";
    let ts_dir: PathBuf = [&env::var("OUT_DIR").unwrap(), ts_name].iter().collect();
    if !ts_dir.exists() {
        fs::create_dir(&ts_dir).unwrap();
    }

    let status = Command::new("tree-sitter")
        .arg("generate")
        .arg(env::current_dir().unwrap().join(grammar_file))
        .current_dir(&ts_dir)
        .status()
        .unwrap();
    assert!(status.success());

    let dir = ts_dir.join("src");
    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .compile(ts_name);
}
