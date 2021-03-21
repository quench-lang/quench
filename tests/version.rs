use std::path::PathBuf;

#[test]
fn test_version() {
    let version = slurp::read_all_to_string("Cargo.toml")
        .unwrap()
        .parse::<toml::Value>()
        .unwrap()
        .get("package")
        .unwrap()
        .get("version")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string();
    for dir in ["editors/code", "tree-sitter-quench"].iter() {
        let path = PathBuf::from(dir).join("package.json");
        assert_eq!(
            version,
            slurp::read_all_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
                .unwrap()
                .get("version")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            "{}",
            path.display(),
        );
    }
}
