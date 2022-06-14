import language from "@quench-lang/tree-sitter/tree-sitter-quench.wasm";
import * as path from "path";
import { LanguageClient } from "vscode-languageclient";

let client;

export const activate = async (context) => {
  client = new LanguageClient(
    "quench",
    "Quench",
    {
      module: context.asAbsolutePath("dist/server.js"),
      args: [context.asAbsolutePath(path.join("dist", language))],
    },
    { documentSelector: [{ scheme: "file", language: "quench" }] }
  );
  client.start();
};

export const deactivate = () => {
  if (!client) {
    return undefined;
  }
  return client.stop();
};
