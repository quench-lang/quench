# Contributing to Moss

This project is still in its very early stages, but feel free to open issues
and/or pull requests regardless. :)

You'll need these tools:

- [Git][]
- [npm][]
- [Emscripten][] or [Docker][] (for building Tree-sitter Wasm)
- [VS Code][] (for testing the VS Code extension)

Clone this repo, `cd` into it, then install dependencies from npm:

```sh
npm i
```

## CLI

```sh
npm run build:tree-sitter
./run.sh packages/examples/hello.moss
```

## Site

### Development

```
npm run build:tree-sitter
npm run site
```

### Production

```
npm run build:tree-sitter
npm run build:site
npm run preview
```

## VS Code extension

```sh
npm run build:tree-sitter
npm run build:vscode
```

Then, from VS Code, press F5 to open a new window with the extension loaded.

[docker]: https://docs.docker.com/get-docker/
[emscripten]: https://emscripten.org/docs/getting_started/downloads.html
[git]: https://git-scm.com/downloads
[npm]: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
[vs code]: https://code.visualstudio.com/download
