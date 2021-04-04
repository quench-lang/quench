use assert_cmd::{assert::OutputAssertExt, prelude::CommandCargoExt};
use goldenfile::Mint;
use pretty_assertions::assert_eq;
use std::{
    collections::BTreeMap,
    ffi::OsStr,
    fs::File,
    io,
    path::{Path, PathBuf},
    process::{Command, Output},
    str,
};
use std::{fs, io::Write};

const GOLDENFILES: &str = "tests/goldenfiles";

#[test]
fn test_help() {
    let mut cmd = Command::cargo_bin("quench").unwrap();
    let assert = cmd.arg("--help").assert().success().stderr("");

    let mut mint = Mint::new(GOLDENFILES);
    let mut file = mint.new_goldenfile("help.txt").unwrap();
    file.write_all(&assert.get_output().stdout).unwrap();
}

fn subcmd<I, S>(name: &str, args: I) -> Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::cargo_bin("quench")
        .unwrap()
        .env("NO_COLOR", "1")
        .arg(name)
        .args(args)
        .output()
        .unwrap()
}

fn to_nonempty_string(bytes: Vec<u8>) -> Option<String> {
    if bytes.is_empty() {
        None
    } else {
        Some(str::from_utf8(&bytes).unwrap().to_string())
    }
}

#[derive(Debug, serde::Deserialize, PartialEq)]
struct Example {
    args: Option<Vec<String>>,
    compile: Option<String>,
    status: Option<i32>,
    out: Option<String>,
    err: Option<String>,
}

fn try_example(stem: String, path: PathBuf, args: Option<Vec<String>>) -> (String, Example) {
    let example = {
        let Output {
            status,
            stdout,
            stderr,
        } = subcmd("compile", &[&path]);
        if status.success() {
            assert!(stderr.is_empty());
            {
                let mut mint = Mint::new("examples");
                let mut file = mint
                    .new_goldenfile(Path::new(&stem).with_extension("js"))
                    .unwrap();
                file.write_all(&stdout).unwrap();
            }

            let Output {
                status,
                stdout,
                stderr,
            } = subcmd("run", {
                let mut full_args = vec![path.to_str().unwrap().to_string()];
                full_args.extend_from_slice(args.as_ref().unwrap_or(&vec![]));
                full_args
            });
            Example {
                args,
                compile: None,
                // we don't just use status.code() here, because we assume there was an exit code
                status: {
                    let code = status.code().unwrap();
                    if code == 0 {
                        None
                    } else {
                        Some(code)
                    }
                },
                out: to_nonempty_string(stdout),
                err: to_nonempty_string(stderr),
            }
        } else {
            assert!(stdout.is_empty());
            Example {
                args,
                compile: to_nonempty_string(stderr),
                status: None,
                out: None,
                err: None,
            }
        }
    };

    (stem, example)
}

fn write_literal(writer: &mut impl Write, key: &str, value: &Option<String>) -> io::Result<()> {
    if let Some(string) = value {
        write!(writer, "  {}: |\n", key)?;
        for line in string.lines() {
            write!(writer, "    {}\n", line)?;
        }
    }
    Ok(())
}

fn write_example(writer: &mut impl Write, name: &str, example: &Example) -> io::Result<()> {
    write!(writer, "{}:\n", name)?;
    let Example {
        args,
        compile,
        status,
        out,
        err,
    } = example;
    if let Some(args) = args {
        write!(writer, "  args:\n")?;
        for arg in args {
            // this will probably eventually have to be made more robust
            write!(writer, "    - {}\n", arg)?;
        }
    }
    write_literal(writer, "compile", compile)?;
    if let Some(code) = status {
        write!(writer, "  status: {}\n", code)?;
    }
    write_literal(writer, "out", out)?;
    write_literal(writer, "err", err)?;
    Ok(())
}

type Examples = BTreeMap<String, Example>;

const EXAMPLES: &str = "examples.yml";

fn read_examples() -> Examples {
    serde_yaml::from_reader(File::open(Path::new(GOLDENFILES).join(EXAMPLES)).unwrap()).unwrap()
}

fn write_examples(writer: &mut impl Write, examples: &Examples) -> io::Result<()> {
    let mut it = examples.iter();
    if let Some((name, example)) = it.next() {
        write_example(writer, name, example)?;
        for (name, example) in it {
            write!(writer, "\n")?;
            write_example(writer, name, example)?;
        }
    }
    Ok(())
}

#[test]
fn test_examples() {
    let actual = read_examples();

    let expected: Examples = fs::read_dir("examples")
        .unwrap()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.extension() == Some(OsStr::new("qn")) {
                let stem = path.file_stem().unwrap().to_str().unwrap().to_string();
                let args = actual
                    .get(&stem)
                    .map_or(None, |example| example.args.clone());
                Some(try_example(stem, path, args))
            } else {
                None
            }
        })
        .collect();

    // assert that the actual YAML file matches what we generate from running the examples; the
    // goldenfile paradigm provides a nice testing UX via its REGENERATE_GOLDENFILES env var, but we
    // need to use a custom write_examples function because yaml-rust doesn't emit literal scalars
    // https://github.com/chyh1990/yaml-rust/pull/132 https://github.com/chyh1990/yaml-rust/pull/137
    write_examples(
        &mut Mint::new(GOLDENFILES).new_goldenfile(EXAMPLES).unwrap(),
        &expected,
    )
    .unwrap();

    // redundant check, to ensure that our write_examples function works correctly; we first re-read
    // the examples file because if REGENERATE_GOLDENFILES is set then it might have been modified
    assert_eq!(read_examples(), expected);
}
