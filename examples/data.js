import Immutable from "https://example.com/quench.js";
const $my_null = null;
const $my_boolean = true;
const $my_other_boolean = false;
const $my_int = BigInt("42");
const $my_negative_int = BigInt("-3");
const $my_string = "hello";
const $my_symbol = Symbol.for("hi_there");
const $my_list = Immutable.List([BigInt("1"), "yes", BigInt("3"), false]);
const $my_empty_list = Immutable.List([]);
const $my_map = Immutable.Map([[Symbol.for("foo"), BigInt("42")], [Immutable.List([Symbol.for("baz"), null]), Symbol.for("qux")], ["bar", Immutable.List([BigInt("-8"), Immutable.Map([]), BigInt("10"), Immutable.List([])])]]);
const $my_empty_map = Immutable.Map([]);
const $main = $_ => (() => {
  console.log($my_int);
})();
$main();
