import { nodeMoss } from "@moss-lang/core";
import * as fs from "fs/promises";
import Parser from "tree-sitter";
import language from "tree-sitter-moss";
import * as url from "url";

const parser = new Parser();
parser.setLanguage(language);
const moss = nodeMoss(parser);
const [, , filename] = process.argv;
const uri = url.pathToFileURL(filename);
moss.setText(uri, await fs.readFile(filename, "utf8"));
console.log(moss.compile(uri));
