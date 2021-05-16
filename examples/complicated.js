import Immutable from "https://example.com/quench.js";
const $main = $_ => (() => {
  console.log("foo");
  console.log(console.log("bar"));
  console.log("👻 ba # not a comment\n  z 🙃");
  console.log(Immutable.List(["foo", "bar", "baz"]));
})();
$main();
