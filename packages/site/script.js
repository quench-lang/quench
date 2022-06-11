import { main } from "@quench-lang/core";
import parserWasmUrl from "../tree-sitter/tree-sitter-quench.wasm?url";
import Parser from "web-tree-sitter";

(async () => {
  await Parser.init();
  const parser = new Parser();
  const Quench = await Parser.Language.load(parserWasmUrl);
  parser.setLanguage(Quench);
  document.getElementById("p").innerText = await main(parser);
})();
