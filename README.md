# Quench [![CI][]][ci link] [![crates.io][]][crates.io link] [![docs.rs][]][docs.rs link] [![rustc version][]][rust release]

A programming language.

## Installation

### Linux

```sh
curl -o quench -L https://github.com/quench-lang/quench/releases/latest/download/quench-linux
chmod +x quench
sudo mv quench /usr/local/bin
```

### macOS 10

```sh
curl -o quench -L https://github.com/quench-lang/quench/releases/latest/download/quench-macos
chmod +x quench
sudo mv quench /usr/local/bin
```

### Windows

Save [quench-windows.exe][] as `quench.exe` somewhere on your PATH.

## Usage

Pass `--help` to the binary you just installed:

```sh
quench --help
```

You will see an example Quench program and instructions on how to run it.

## Editors

Support (and instructions) for specific text editors can be found in the
[editors][] folder.

## Development

### Prerequisites

- [Rust][]

### Basics

Clone this repo, `cd` into it, and run an example using `cargo run`:

```sh
cargo run -- examples/hello.qn
```

Or if you want to use the `#!/usr/bin/env quench` shebang, first install:

```sh
cargo install --locked --path .
```

Then run the example directly:

```sh
examples/hello.qn
```

### Other

If you want to modify the grammar, be sure to follow the instructions in the
[tree-sitter-quench][] folder.

## License

This repository is released under the [MIT License](/LICENSE).

_I am providing code in the repository to you under an open source license.
Because this is my personal repository, the license you receive to my code is
from me and not my employer (Facebook)._

[ci]: https://github.com/quench-lang/quench/actions/workflows/ci.yml/badge.svg
[ci link]: https://github.com/quench-lang/quench/actions/workflows/ci.yml
[crates.io]: https://img.shields.io/crates/v/quench
[crates.io link]: https://crates.io/crates/quench
[docs.rs]: https://docs.rs/quench/badge.svg
[docs.rs link]: https://docs.rs/quench
[editors]: /editors
[quench-windows.exe]: https://github.com/quench-lang/quench/releases/latest/download/quench-windows.exe
[tree-sitter-quench]: /tree-sitter-quench
[rust]: https://www.rust-lang.org/tools/install
[rust release]: https://github.com/rust-lang/rust/blob/1.48.0/RELEASES.md#libraries
[rustc version]: https://img.shields.io/badge/rustc-1.48+-lightgray.svg
