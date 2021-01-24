use anyhow::Result;
use assert_cmd::prelude::*;
use goldenfile::Mint;
use std::io::Write;
use std::process::Command;

#[test]
fn help() -> Result<()> {
    let mut cmd = Command::cargo_bin("quench")?;
    let assert = cmd.arg("--help").assert().success().stderr("");

    let mut mint = Mint::new("tests/goldenfiles");
    let mut file = mint.new_goldenfile("help.txt")?;
    file.write_all(&assert.get_output().stdout)?;

    Ok(())
}
