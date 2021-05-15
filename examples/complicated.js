import * as Immutable from "https://deno.land/x/immutable@4.0.0-rc.12-deno/mod.ts";
var main = function (_) {
  return (function () {
    console.log("foo");
    console.log(console.log("bar"));
    console.log("👻 ba # not a comment\n  z 🙃");
    console.log(Immutable.List(["foo", "bar", "baz"]));
  })();
};
main();
