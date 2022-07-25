import * as astring from "astring";

const mangle = (name) => `$${name}`;

const compileIdentifier = (id) => {
  const name = id.text;
  switch (name) {
    case "print": {
      return {
        type: "MemberExpression",
        object: { type: "Identifier", name: "console" },
        property: { type: "Identifier", name: "log" },
        computed: false,
      };
    }
    case "args": {
      return {
        type: "CallExpression",
        callee: {
          type: "MemberExpression",
          object: {
            type: "MemberExpression",
            object: { type: "Identifier", name: "process" },
            property: { type: "Identifier", name: "argv" },
            computed: false,
          },
          property: { type: "Identifier", name: "slice" },
          computed: false,
        },
        arguments: [{ type: "Literal", value: 2 }],
      };
    }
    default: {
      return { type: "Identifier", name: mangle(name) };
    }
  }
};

const compileSymbol = (sym) => ({
  type: "CallExpression",
  callee: {
    type: "MemberExpression",
    object: { type: "Identifier", name: "Symbol" },
    property: { type: "Identifier", name: "for" },
    computed: false,
  },
  arguments: [{ type: "Literal", value: sym.text.slice(1) }],
});

const main = "main";

class Moss {
  constructor({ parser, childForFieldName, childrenForFieldName }) {
    this.parser = parser;
    this.childForFieldName = childForFieldName;
    this.childrenForFieldName = childrenForFieldName;
    this.trees = new Map();
  }

  setText(uri, text) {
    this.trees.set(uri, this.parser.parse(text));
  }

  close(uri) {
    this.trees.delete(uri);
  }

  getTreeRoot(uri) {
    return this.trees.get(uri).rootNode;
  }

  // compilation

  compileStatement(stmt) {
    switch (stmt.type) {
      case "declaration": {
        return this.compileDeclaration(stmt);
      }
      case "expression_statement": {
        return {
          type: "ExpressionStatement",
          expression: this.compileExpression(
            this.childForFieldName(stmt, "expression")
          ),
        };
      }
    }
  }

  compileBlock(block) {
    const body = this.childrenForFieldName(block, "statement").map((stmt) =>
      this.compileStatement(stmt)
    );
    const expr = this.childForFieldName(block, "expression");
    if (expr) {
      body.push({
        type: "ReturnStatement",
        argument: this.compileExpression(expr),
      });
    }
    return {
      type: "CallExpression",
      callee: {
        type: "ArrowFunctionExpression",
        id: null,
        params: [],
        body: { type: "BlockStatement", body },
        generator: false,
        expression: true,
      },
      arguments: [],
    };
  }

  compileEntry(entry) {
    return {
      type: "ArrayExpression",
      elements: [
        this.compileExpression(this.childForFieldName(entry, "key")),
        this.compileExpression(this.childForFieldName(entry, "value")),
      ],
    };
  }

  compileLiteral(lit) {
    switch (lit.type) {
      case "null": {
        return { type: "Literal", value: null };
      }
      case "boolean":
      case "integer": {
        return { type: "Literal", value: JSON.parse(lit.text) };
      }
      case "string": {
        return { type: "Literal", value: lit.text.slice(1, -1) };
      }
      case "symbol": {
        return compileSymbol(lit);
      }
      case "list": {
        return {
          type: "CallExpression",
          callee: {
            type: "MemberExpression",
            object: { type: "Identifier", name: "Immutable" },
            property: { type: "Identifier", name: "List" },
            computed: false,
          },
          arguments: [
            {
              type: "ArrayExpression",
              elements: this.childrenForFieldName(lit, "item").map((item) =>
                this.compileExpression(item)
              ),
            },
          ],
        };
      }
      case "map": {
        return {
          type: "CallExpression",
          callee: {
            type: "MemberExpression",
            object: { type: "Identifier", name: "Immutable" },
            property: { type: "Identifier", name: "Map" },
            computed: false,
          },
          arguments: [
            {
              type: "ArrayExpression",
              elements: this.childrenForFieldName(lit, "entry").map((entry) =>
                this.compileEntry(entry)
              ),
            },
          ],
        };
      }
    }
  }

  compileExpression(expr) {
    switch (expr.type) {
      case "parenthesized": {
        return this.compileExpression(
          this.childForFieldName(expr, "expression")
        );
      }
      case "identifier": {
        return compileIdentifier(expr);
      }
      case "block": {
        return this.compileBlock(expr);
      }
      case "call": {
        return {
          type: "CallExpression",
          callee: this.compileExpression(
            this.childForFieldName(expr, "function")
          ),
          arguments: [
            this.compileExpression(this.childForFieldName(expr, "argument")),
          ],
        };
      }
      case "function": {
        return {
          type: "ArrowFunctionExpression",
          id: null,
          params: [
            {
              type: "Identifier",
              name: mangle(this.childForFieldName(expr, "parameter").text),
            },
          ],
          body: this.compileExpression(this.childForFieldName(expr, "body")),
          generator: false,
          expression: true,
        };
      }
      case "index": {
        return {
          type: "CallExpression",
          callee: {
            type: "MemberExpression",
            object: this.compileExpression(
              this.childForFieldName(expr, "collection")
            ),
            property: { type: "Identifier", name: "get" },
            computed: false,
          },
          arguments: [
            this.compileExpression(this.childForFieldName(expr, "key")),
          ],
        };
      }
      case "field": {
        return {
          type: "CallExpression",
          callee: {
            type: "MemberExpression",
            object: this.compileExpression(this.childForFieldName(expr, "map")),
            property: { type: "Identifier", name: "get" },
            computed: false,
          },
          arguments: [this.compileSymbol(this.childForFieldName(expr, "key"))],
        };
      }
      default: {
        return this.compileLiteral(expr);
      }
    }
  }

  compileDeclaration(decl) {
    return {
      type: "VariableDeclaration",
      declarations: [
        {
          type: "VariableDeclarator",
          id: {
            type: "Identifier",
            name: mangle(this.childForFieldName(decl, "name").text),
          },
          init: this.compileExpression(this.childForFieldName(decl, "value")),
        },
      ],
      kind: "const",
    };
  }

  compile(uri) {
    const decls = this.childrenForFieldName(
      this.getTreeRoot(uri),
      "declaration"
    );
    const body = [
      {
        type: "ImportDeclaration",
        specifiers: [
          {
            type: "ImportDefaultSpecifier",
            local: { type: "Identifier", name: "Immutable" },
          },
        ],
        source: { type: "Literal", value: "immutable" },
      },
      ...decls.map((decl) => this.compileDeclaration(decl)),
    ];
    if (
      decls.some((decl) => this.childForFieldName(decl, "name").text === main)
    ) {
      body.push({
        type: "ExpressionStatement",
        expression: {
          type: "CallExpression",
          callee: { type: "Identifier", name: mangle(main) },
          arguments: [],
        },
      });
    }
    return astring.generate({ type: "Program", sourceType: "module", body });
  }
}

export const nodeMoss = (parser) =>
  new Moss({
    parser,
    childForFieldName: (node, fieldName) => node[`${fieldName}Node`],
    childrenForFieldName: (node, fieldName) => node[`${fieldName}Nodes`],
  });

export const webMoss = (parser) =>
  new Moss({
    parser,

    childForFieldName: (node, fieldName) => node.childForFieldName(fieldName),

    // https://github.com/tree-sitter/tree-sitter/issues/601#issuecomment-994675441
    childrenForFieldName: (node, fieldName) => {
      const treeCursor = node.walk();
      treeCursor.gotoFirstChild();

      const ret = [];

      let hasNext = true;

      while (hasNext) {
        if (treeCursor.currentFieldName() === fieldName) {
          ret.push(treeCursor.currentNode());
        }

        hasNext = treeCursor.gotoNextSibling();
      }

      return ret;
    },
  });
