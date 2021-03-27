# [VS Code extension for Quench][marketplace]

This extension provides support for the [Quench][] language via a [language
server][lsp], which it automatically downloads if you don't already have it
installed. To use, simply install this extension, open any file whose name ends
with `.qn`, and click the "Download" button if prompted.

![screnshot of hello.qn in VS Code](hello.png)

## Development

### Prerequisites

- [Node][]
- [VS Code][]

### Basics

The below instructions assume that your VS Code workspace is (the root of) the
Quench repo but your shell cwd is this directory (`editors/code` in the repo).

First install dependencies:

```sh
npm install
```

Then compile:

```sh
npm run compile
```

Next, from VS Code, press F5 to open a new window with this extension loaded. To
avoid developing with a mismatched version of Quench itself, you may also need
to build Quench from source and change the `quench.server.path` setting to point
to the binary you've built. Then open a Quench source file (such as
`examples/hello.qn` from this repository) to activate it as normal.

[lsp]: https://microsoft.github.io/language-server-protocol/
[marketplace]: https://marketplace.visualstudio.com/items?itemName=quench.quench
[node]: https://github.com/nvm-sh/nvm#install--update-script
[quench]: https://github.com/quench-lang/quench
[vs code]: https://code.visualstudio.com/download
