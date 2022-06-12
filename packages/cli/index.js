import { QuenchParser } from "@quench-lang/core";
import Quench from "@quench-lang/tree-sitter";
import * as fs from "fs/promises";
import Parser from "tree-sitter";

const parser = new Parser();
parser.setLanguage(Quench);
console.log(
  new QuenchParser(
    parser,
    await fs.readFile(process.argv[2], "utf8")
  ).astString()
);
