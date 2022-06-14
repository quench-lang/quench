import { Quench } from "@quench-lang/core";
import {
  createConnection,
  ProposedFeatures,
  TextDocumentSyncKind,
  SemanticTokensRequest,
  SemanticTokenTypes,
} from "vscode-languageserver";

export const startServer = (makeParser) => {
  const connection = createConnection(ProposedFeatures.all);

  let quench;

  connection.onInitialize(async () => {
    quench = new Quench(await makeParser());

    return {
      capabilities: {
        textDocumentSync: TextDocumentSyncKind.Full,
        semanticTokensProvider: {
          legend: { tokenTypes, tokenModifiers: [] },
          range: false,
          full: { delta: false },
        },
      },
    };
  });

  const tokenTypes = [
    SemanticTokenTypes.comment,
    SemanticTokenTypes.enumMember,
    SemanticTokenTypes.number,
    SemanticTokenTypes.string,
    SemanticTokenTypes.property,
    SemanticTokenTypes.variable,
  ];

  connection.onDidOpenTextDocument(({ textDocument: { uri, text } }) => {
    quench.setText(uri, text);
  });

  connection.onDidChangeTextDocument(
    ({ textDocument: { uri }, contentChanges: [{ text }] }) => {
      quench.setText(uri, text);
    }
  );

  connection.onDidCloseTextDocument(({ textDocument: { uri } }) => {
    quench.close(uri);
  });

  const nodeTypeMap = {
    boolean: SemanticTokenTypes.enumMember,
    comment: SemanticTokenTypes.comment,
    integer: SemanticTokenTypes.number,
    null: SemanticTokenTypes.enumMember,
    string: SemanticTokenTypes.string,
    symbol: SemanticTokenTypes.property,
    identifier: SemanticTokenTypes.variable,
  };

  const tokenTypeMap = new Map(tokenTypes.map((x, i) => [x, i]));

  connection.onRequest(
    SemanticTokensRequest.type,
    ({ textDocument: { uri } }) => {
      const tokens = [];

      const gatherTokens = (node) => {
        const tokenType = nodeTypeMap[node.type];
        if (tokenType === undefined) {
          node.children.forEach(gatherTokens);
        } else {
          const start = node.startPosition;
          const end = node.endPosition;
          let column = start.column;

          const lines = node.text.split(/\r?\n/);
          lines.pop();
          lines.forEach((line, i) => {
            tokens.push({
              line: start.row + i,
              start: column,
              length: line.length,
              tokenType,
            });
            column = 0;
          });

          tokens.push({
            line: end.row,
            start: column,
            length: end.column - column,
            tokenType,
          });
        }
      };

      gatherTokens(quench.getTreeRoot(uri));

      const data = [];
      if (tokens.length > 0) {
        const [first, ...rest] = tokens;
        data.push(
          first.line,
          first.start,
          first.length,
          tokenTypeMap.get(first.tokenType),
          0
        );
        let last_line = first.line;
        let last_start = first.start;
        for (const token of rest) {
          data.push(
            token.line - last_line,
            token.line === last_line ? token.start - last_start : token.start,
            token.length,
            tokenTypeMap.get(token.tokenType),
            0
          );
          last_line = token.line;
          last_start = token.start;
        }
      }

      return { data };
    }
  );

  connection.listen();
};
