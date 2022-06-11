import * as fs from "fs/promises";
import * as path from "path";

const filename = "tree-sitter.wasm";
await fs.copyFile(
  path.join("../../node_modules/web-tree-sitter", filename),
  path.join("dist", filename)
);
