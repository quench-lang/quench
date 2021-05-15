pub const ASTRING: &str = concat!(
    "https://github.com/quench-lang/quench/raw/",
    env!("VERGEN_GIT_SHA"),
    "/jsdeps/node_modules/astring/dist/astring.min.js",
);
pub const ASTRING_SOURCE: &str = include_str!("../jsdeps/node_modules/astring/dist/astring.min.js");

pub const STDLIB_PLACEHOLDER: &str = "https://example.com/quench.js";
pub const STDLIB: &str = concat!(
    "https://github.com/quench-lang/quench/raw/",
    env!("VERGEN_GIT_SHA"),
    "/src/stdlib.js",
);
pub const STDLIB_SOURCE: &str = include_str!("stdlib.js");

pub const IMMUTABLE_SPECIFIED: &str =
    "https://deno.land/x/immutable@4.0.0-rc.12/node_modules/immutable/dist/immutable.es.js";
pub const IMMUTABLE_FOUND: &str = concat!(
    "https://github.com/quench-lang/quench/raw/",
    env!("VERGEN_GIT_SHA"),
    "/jsdeps/node_modules/immutable/dist/immutable.es.js",
);
pub const IMMUTABLE_SOURCE: &str =
    include_str!("../jsdeps/node_modules/immutable/dist/immutable.es.js");
