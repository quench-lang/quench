import * as lsp from "@quench-lang/lsp";
import Parser from "web-tree-sitter";

lsp.startServer(async () => {
  await Parser.init();
  const parser = new Parser();
  parser.setLanguage(await Parser.Language.load(process.argv[2]));
  return parser;
});
