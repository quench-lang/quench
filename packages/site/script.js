import { QuenchParser } from "@quench-lang/core";
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
  const state = new QuenchParser(parser, code.getValue());

  const setAst = (preface) => {
    ast.setValue(`${preface}\n\n${state.astString()}`);
  };

  setAst("initial text");
  code.onDidChangeModelContent(({ changes }) => {
    const newSourceCode = code.getValue();
    let description = `${changes.length} change`;

    const t0 = performance.now();
    if (changes.length === 1) {
      const [
        {
          range: { startLineNumber, startColumn, endLineNumber, endColumn },
          rangeLength,
          text,
          rangeOffset,
        },
      ] = changes;

      const lastNewline = text.lastIndexOf("\n");

      state.applyEdit(
        {
          startIndex: rangeOffset,
          oldEndIndex: rangeOffset + rangeLength,
          newEndIndex: rangeOffset + text.length,
          startPosition: { row: startLineNumber - 1, column: startColumn - 1 },
          oldEndPosition: { row: endLineNumber - 1, column: endColumn - 1 },
          newEndPosition:
            lastNewline < 0
              ? {
                  row: startLineNumber - 1,
                  column: startColumn - 1 + text.length,
                }
              : {
                  row: startLineNumber - 1 + text.match(/\n/g).length,
                  column: text.length - lastNewline - 1,
                },
        },
        newSourceCode
      );

      description += ": applied edit";
    } else {
      state.parse(newSourceCode);
      description += "s: re-parsed from scratch";
    }
    const t1 = performance.now();

    setAst(`${description} in ${t1 - t0} ms`);
  });
})();
