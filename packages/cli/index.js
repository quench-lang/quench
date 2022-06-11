import { main } from "@quench-lang/core";
import Quench from "@quench-lang/tree-sitter";
import Parser from "tree-sitter";

const parser = new Parser();
parser.setLanguage(Quench);
console.log(await main(parser));
