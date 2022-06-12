export const main = async (parser, sourceCode) => {
  const tree = parser.parse(sourceCode);
  return tree.rootNode.toString();
};
