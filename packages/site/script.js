import { main } from "@quench-lang/core";
import helloExample from "@quench-lang/examples/hello.qn?raw";
import parseWasmUrl from "@quench-lang/tree-sitter/tree-sitter-quench.wasm?url";
import * as monaco from "monaco-editor";
import Parser from "web-tree-sitter";

console.log(parseWasmUrl);

const options = { automaticLayout: true, theme: "vs-dark", wordWrap: "on" };

const code = monaco.editor.create(document.getElementById("code"), {
  ...options,
  value: helloExample,
});
const ast = monaco.editor.create(document.getElementById("ast"), {
  ...options,
  readOnly: true,
});

(async () => {
  await Parser.init();
  const parser = new Parser();
  const Quench = await Parser.Language.load(parseWasmUrl);
  parser.setLanguage(Quench);

  const update = async () => {
    ast.setValue(await main(parser, code.getValue()));
  };

  update();
  code.onDidChangeModelContent(update);
})();
