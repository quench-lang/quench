# Quench

A programming language.

## Prerequisites

- [Rust][]

## Usage

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

## Editors

Support (and instructions) for specific text editors can be found in the
[editors][] folder.

## Development

If you want to modify the grammar, be sure to follow the instructions in the
[tree-sitter-quench][] folder.

## License

This repository is released under the [MIT License](/LICENSE).

_I am providing code in the repository to you under an open source license.
Because this is my personal repository, the license you receive to my code is
from me and not my employer (Facebook)._

[editors]: /editors
[tree-sitter-quench]: /tree-sitter-quench
[rust]: https://www.rust-lang.org/tools/install
