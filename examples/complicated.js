import Immutable from "https://example.com/quench.js";
var main = function (_) {
  return (function () {
    console.log("foo");
    console.log(console.log("bar"));
    console.log("👻 ba # not a comment\n  z 🙃");
    console.log(Immutable.List(["foo", "bar", "baz"]));
  })();
};
main();
