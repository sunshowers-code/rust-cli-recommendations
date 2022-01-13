# Picking a command-line argument parser

There are a number of different command-line parsers for Rust programs. However, projects SHOULD use [**clap**](https://crates.io/crates/clap).

**Why?**
* clap is actively maintained: as of January 2022, clap just came out with a [v3 release]().
* clap is the most popular command-line parsing library for Rust, which means that there's an existing ecosystem of projects around clap.
* clap comes with a number of extra features, such as suggestions based on [Jaro–Winkler distance](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance) and full configurability of [commands](https://docs.rs/clap/latest/clap/enum.AppSettings.html) and [arguments](https://docs.rs/clap/latest/clap/enum.ArgSettings.html)
* There are a number of standard conventions for Unix CLIs: see [this comment](https://github.com/google/argh/issues/3#issuecomment-581144181) by [Stephen Sokolow](https://github.com/ssokolow). Another actively maintained project, [argh](https://github.com/google/argh), does not target Unix platforms and so does not support all of these conventions.

**Why not?**

* clap pulls in several dependencies and takes quite a while to build.
* clap increases binary size significantly.
* clap is a complex parser with many different options, which can be overwhelming. I've found use for many of these options.
* The latest version of clap as of January 2022 is 3.0.6. This version currently has a Minimum Supported Rust Version (MSRV) of Rust 1.54; I personally do not consider this to be a negative but there's [some discussions about it](https://github.com/clap-rs/clap/issues/3267). For now, a workaround is to use clap v2.

## Automatically deriving fields

Projects MAY turn on the `derive` feature in clap and use a declarative model to define command-line arguments. (The `derive` feature is new to v3---users of clap v2 can use [structopt](https://crates.io/crates/structopt), which `clap_derive` is based on.)

For example:

```rust
use clap::Parser;

/// A very simple utility to search for a string across multiple files.
#[derive(Debug, Parser)]
pub struct GrepApp {
    /// Suppress normal output; instead print the name of each input file from which output
    /// would normally have been printed.  Scanning each input file stops upon first match.
    #[clap(long, short = "l")]
    files_with_matches: bool,

    /// Search string
    search_str: String,

    /// Input files
    files: Vec<String>,
}
```

The doc comments are processed as help text by clap.

**Why?**
* Derive-style arguments are significantly easier to read, write, and modify.
* Derive-style components can be written once, and reused across multiple commands.

**Why not?**
* The derive macro is an optional feature that pulls in extra dependencies and increase build times.
* The derive macro can be a bit magical. Looking at [the source code of clap_derive](https://github.com/clap-rs/clap/blob/master/clap_derive/src/lib.rs) may be useful sometimes.

## Alternatives to clap

* [argh](https://github.com/google/argh): Google's argument parser. Actively maintained, but targets the Fuchsia OS rather than Unix platforms, so missing several crucial features.
* [pico-args](https://github.com/RazrFalcon/pico-args): Quick to compile and negligible impact on binary size, but does not include help generation, derive support, or as many config flags as clap.
* [gumdrop](https://crates.io/crates/gumdrop): a simple argument parser with derive support. Somewhat less popular than clap, and doesn't support deserializing directly to domain types (clap [does](https://github.com/clap-rs/clap/blob/v3.0.6/examples/derive_ref/README.md#arg-types)).
* Writing your own by hand: applications SHOULD NOT do this. You're probably going to get it wrong and annoy people---but it's an option if you must.
