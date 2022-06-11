export const main = async (parser) => {
  const sourceCode = 'main := _ => print "Hello, world!";';
  const tree = parser.parse(sourceCode);
  return tree.rootNode.toString();
};
