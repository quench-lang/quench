use std::path::PathBuf;

fn main() {
    let ts_name = "tree-sitter-quench";
    let dir: PathBuf = [ts_name, "src"].iter().collect();
    println!("cargo:rerun-if-changed={}", dir.display());
    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .compile(ts_name);
}
