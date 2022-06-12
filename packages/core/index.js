export class QuenchParser {
  constructor(parser, sourceCode) {
    this.parser = parser;
    this.parse(sourceCode);
  }

  parse(sourceCode) {
    this.tree = this.parser.parse(sourceCode);
  }

  applyEdit(edit, newSourceCode) {
    this.tree.edit(edit);
    this.tree = this.parser.parse(newSourceCode, this.tree);
  }

  astString() {
    return this.tree.rootNode.toString();
  }
}
