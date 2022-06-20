# Contributing to Moss

This project is still in its very early stages, but feel free to open issues
and/or pull requests regardless. :)

You'll need these tools:

- [Git][]
- [Yarn][]
- [Emscripten][] or [Docker][] (for building Tree-sitter Wasm)
- [VS Code][] (for testing the VS Code extension)

Clone this repo, `cd` into it, then install dependencies from npm:

```sh
yarn
```

## CLI

```sh
yarn build:tree-sitter
./run.sh packages/examples/hello.moss
```

## Site

### Development

```
yarn build:tree-sitter
yarn site
```

### Production

```
yarn build:tree-sitter
yarn build:site
yarn preview
```

## VS Code extension

```sh
yarn build:tree-sitter
yarn build:vscode
```

Then, from VS Code, press F5 to open a new window with the extension loaded.

[docker]: https://docs.docker.com/get-docker/
[emscripten]: https://emscripten.org/docs/getting_started/downloads.html
[git]: https://git-scm.com/downloads
[vs code]: https://code.visualstudio.com/download
[yarn]: https://classic.yarnpkg.com/lang/en/docs/install/
