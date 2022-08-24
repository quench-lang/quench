import { webMoss } from "@moss-lang/core";
import * as lsp from "@moss-lang/lsp";
import Parser from "web-tree-sitter";

lsp.startServer({
  makeMoss: async () => {
    await Parser.init();
    const parser = new Parser();
    parser.setLanguage(await Parser.Language.load(process.argv[2]));
    return webMoss(parser);
  },
});
