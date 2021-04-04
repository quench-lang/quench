use assert_cmd::{assert::OutputAssertExt, prelude::CommandCargoExt};
use goldenfile::Mint;
use std::{
    ffi::OsStr,
    path::Path,
    process::{Command, Output},
};
use std::{fs, io::Write};

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("quench").unwrap();
    let assert = cmd.arg("--help").assert().success().stderr("");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file = mint.new_goldenfile("help.txt").unwrap();
    file.write_all(&assert.get_output().stdout).unwrap();
}

fn assert_expected(subcmd: &str, src: &Path) {
    let mut cmd = Command::cargo_bin("quench").unwrap();
    let assert = cmd.env("NO_COLOR", "1").arg(subcmd).arg(&src).assert();
    let Output {
        status,
        stdout,
        stderr,
    } = assert.get_output();

    if stderr.is_empty() {
        assert!(status.success());
    } else {
        assert!(!status.success());
    }

    let dir = src.parent().unwrap().join(subcmd);
    let txt_path = src.with_extension("txt");
    let txt_name = Path::new(txt_path.file_name().unwrap());

    let mut out_mint = Mint::new(dir.join("out"));
    let mut out = out_mint.new_goldenfile(txt_name).unwrap();
    out.write_all(stdout).unwrap();

    let mut err_mint = Mint::new(dir.join("err"));
    let mut err = err_mint.new_goldenfile(txt_name).unwrap();
    err.write_all(stderr).unwrap();
}

#[test]
fn test_examples() {
    for entry in fs::read_dir("examples").unwrap() {
        let path = entry.unwrap().path();
        if path.extension() == Some(OsStr::new("qn")) {
            assert_expected("compile", &path);
            assert_expected("run", &path);
        }
    }
}
