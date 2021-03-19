use assert_cmd::{assert::OutputAssertExt, prelude::CommandCargoExt};
use goldenfile::Mint;
use std::{ffi::OsStr, process::Command};
use std::{fs, io::Write};

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("quench").unwrap();
    let assert = cmd.arg("--help").assert().success().stderr("");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file = mint.new_goldenfile("help.txt").unwrap();
    file.write_all(&assert.get_output().stdout).unwrap();
}

#[test]
fn test_examples() {
    let dir = "examples";
    let mut mint = Mint::new(dir);

    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension() == Some(OsStr::new("qn")) {
            let mut cmd = Command::cargo_bin("quench").unwrap();
            let assert = cmd.arg(&path).assert().success().stderr("");

            let mut file = mint
                .new_goldenfile(path.with_extension("txt").file_name().unwrap())
                .unwrap();
            file.write_all(&assert.get_output().stdout).unwrap();
        }
    }
}
