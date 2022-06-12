# Quench [![CI](https://github.com/quench-lang/quench/actions/workflows/ci.yml/badge.svg)](https://github.com/quench-lang/quench/actions/workflows/ci.yml) [![license](https://img.shields.io/github/license/penrose/penrose)](LICENSE)

A programming language.

## Usage

Try it in your browser, no installation required: https://quench-lang.org/

## Development

### Prerequisites

- [Yarn][]
- [Docker][] or [Emscripten][] (for building Tree-sitter Wasm)

### Basics

Clone this repo, `cd` into it, then:

```sh
yarn
```

### CLI

```sh
yarn build:tree-sitter
yarn cli packages/examples/hello.qn
```

### Site (development)

```
yarn build:tree-sitter
yarn site
```

### Site (production)

```
yarn build
yarn preview
```

## License

This repository is released under the [MIT License](/LICENSE).

[docker]: https://docs.docker.com/get-docker/
[emscripten]: https://emscripten.org/docs/getting_started/downloads.html
[yarn]: https://classic.yarnpkg.com/lang/en/docs/install/
