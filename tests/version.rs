use comrak::ComrakOptions;
use regex::Regex;
use std::{collections::HashSet, path::PathBuf, str};

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
    for dir in ["jsdeps", "editors/code", "tree-sitter-quench"].iter() {
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

#[test]
fn test_minimum_rustc() {
    let readme_versions: HashSet<String> = {
        let arena = comrak::Arena::new();
        let heading = comrak::parse_document(
            &arena,
            &slurp::read_all_to_string("README.md").unwrap(),
            &ComrakOptions::default(),
        )
        .first_child()
        .unwrap();
        assert!(matches!(
            heading.data.borrow().value,
            comrak::nodes::NodeValue::Heading(comrak::nodes::NodeHeading { level: 1, .. }),
        ));

        let mut versions = HashSet::new();
        let re = Regex::new(r"rustc-(\d+\.\d+)\+").unwrap();
        for node in heading.children() {
            if let comrak::nodes::NodeValue::Link(comrak::nodes::NodeLink { .. }) =
                &node.data.borrow().value
            {
                if let Some(inner) = node.first_child() {
                    if let comrak::nodes::NodeValue::Image(comrak::nodes::NodeLink {
                        url, ..
                    }) = &inner.data.borrow().value
                    {
                        // if we got here, node is a badge
                        if let Some(m) = re.captures(str::from_utf8(url).unwrap()) {
                            versions.insert(String::from(&m[1]));
                        }
                    }
                }
            }
        }
        versions
    };
    assert!(!readme_versions.is_empty());

    let ci_versions: HashSet<String> = {
        let re = Regex::new(r"^(\d+\.\d+)\.\d+$").unwrap();
        serde_yaml::from_str::<serde_yaml::Value>(
            &slurp::read_all_to_string(".github/workflows/ci.yml").unwrap(),
        )
        .unwrap()
        .get("jobs")
        .unwrap()
        .get("rust")
        .unwrap()
        .get("strategy")
        .unwrap()
        .get("matrix")
        .unwrap()
        .get("rust")
        .unwrap()
        .as_sequence()
        .unwrap()
        .iter()
        .filter_map(|version| re.captures(version.as_str().unwrap()))
        .map(|m| String::from(&m[1]))
        .collect()
    };
    assert!(
        readme_versions.is_subset(&ci_versions),
        "README rustc versions {:?} should be a subset of CI Rust versions {:?}",
        readme_versions,
        ci_versions,
    );
}
