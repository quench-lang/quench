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

const compileStatement = (stmt) => {
  switch (stmt.type) {
    case "declaration": {
      return compileDeclaration(stmt);
    }
    case "expression_statement": {
      return {
        type: "ExpressionStatement",
        expression: compileExpression(stmt.expressionNode),
      };
    }
  }
};

const compileBlock = (block) => {
  const body = block.statementNodes.map(compileStatement);
  const expr = block.expressionNode;
  if (expr) {
    body.push({ type: "ReturnStatement", argument: compileExpression(expr) });
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

const compileEntry = (entry) => ({
  type: "ArrayExpression",
  elements: [
    compileExpression(entry.keyNode),
    compileExpression(entry.valueNode),
  ],
});

const compileLiteral = (lit) => {
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
            elements: lit.itemNodes.map(compileExpression),
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
            elements: lit.entryNodes.map(compileEntry),
          },
        ],
      };
    }
  }
};

const compileExpression = (expr) => {
  switch (expr.type) {
    case "parenthesized": {
      return compileExpression(expr.expressionNode);
    }
    case "identifier": {
      return compileIdentifier(expr);
    }
    case "block": {
      return compileBlock(expr);
    }
    case "call": {
      return {
        type: "CallExpression",
        callee: compileExpression(expr.functionNode),
        arguments: [compileExpression(expr.argumentNode)],
      };
    }
    case "function": {
      return {
        type: "ArrowFunctionExpression",
        id: null,
        params: [{ type: "Identifier", name: mangle(expr.parameterNode.text) }],
        body: compileExpression(expr.bodyNode),
        generator: false,
        expression: true,
      };
    }
    case "index": {
      return {
        type: "CallExpression",
        callee: {
          type: "MemberExpression",
          object: compileExpression(expr.collectionNode),
          property: { type: "Identifier", name: "get" },
          computed: false,
        },
        arguments: [compileExpression(expr.keyNode)],
      };
    }
    case "field": {
      return {
        type: "CallExpression",
        callee: {
          type: "MemberExpression",
          object: compileExpression(expr.mapNode),
          property: { type: "Identifier", name: "get" },
          computed: false,
        },
        arguments: [compileSymbol(expr.keyNode)],
      };
    }
    default: {
      return compileLiteral(expr);
    }
  }
};

const compileDeclaration = (decl) => ({
  type: "VariableDeclaration",
  declarations: [
    {
      type: "VariableDeclarator",
      id: { type: "Identifier", name: mangle(decl.nameNode.text) },
      init: compileExpression(decl.valueNode),
    },
  ],
  kind: "const",
});

const main = "main";

export class Moss {
  constructor(parser) {
    this.parser = parser;
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

  compile(uri) {
    const decls = this.getTreeRoot(uri).declarationNodes;
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
      ...decls.map(compileDeclaration),
    ];
    if (decls.some((decl) => decl.nameNode.text === main)) {
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
