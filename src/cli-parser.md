# Picking an argument parser

When you're writing a Rust command-line application, one of the first things you'll have to do is to figure out how to parse command-line inputs.
There are a number of different command-line parsers for Rust programs. However, projects *should* use [**clap**](https://crates.io/crates/clap).

**Why?**

* clap is actively maintained: as of January 2022, clap just came out with a [v3 release]().
* clap is the most popular command-line parsing library for Rust, which means that there's an existing ecosystem of projects around clap.
* clap comes with a number of extra features, such as suggestions based on [Jaro–Winkler distance](https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance) and full configurability of [commands](https://docs.rs/clap/latest/clap/enum.AppSettings.html) and [arguments](https://docs.rs/clap/latest/clap/enum.ArgSettings.html).
* There are a number of standard conventions for Unix CLIs: see [this comment](https://github.com/google/argh/issues/3#issuecomment-581144181) by [Stephen Sokolow](https://github.com/ssokolow). clap supports all of them. Another actively maintained project, [argh](https://github.com/google/argh), does not target Unix platforms and so does not support all of these conventions.

**Reasons against using clap**

* clap pulls in several dependencies and takes quite a while to build.
* clap increases binary size significantly.
* clap is a complex parser with many different options. I've found uses for most of them, but they can be overwhelming.
* clap version 3 currently has a Minimum Supported Rust Version (MSRV) of Rust 1.54; I personally do not consider this to be a negative but there's [some discussions about it](https://github.com/clap-rs/clap/issues/3267). For now, a workaround is to use version 2 of clap, which supports most of the features that clap version 3 does.

## Automatically deriving arguments

Projects *may* turn on the `derive` feature in clap and use a declarative model to define command-line arguments. (The `derive` feature is new to v3---users of clap v2 can use [structopt](https://crates.io/crates/structopt), which `clap_derive` is based on.)

For example:

```rust,noplaypen
{{#rustdoc_include ../code/cli-parser/src/bin/grep-app.rs:definition}}
```

The doc comments are processed as help text by clap. Here's what the help text looks like:

```rust,noplaypen
{{#rustdoc_include ../code/cli-parser/src/bin/grep-app.rs:grep-help}}
```

**Why?**

* Derive-style arguments are significantly easier to read, write, and modify.
* Derive-style components can be written once, and reused across multiple commands.

**Why not?**

* The derive macro is an optional feature that pulls in extra dependencies and increases build times.
* The derive macro can be a bit magical. Looking at [the source code of clap_derive](https://github.com/clap-rs/clap/blob/master/clap_derive/src/lib.rs), or the generated output with [cargo-expand](https://crates.io/crates/cargo-expand), may be useful.
* The derive macro is less flexible than the builder API. For example, for an argument used multiple times like `-v -v -v`, the builder API can tell you exactly which position each `-v` was used in. The derive macro can only tell you how many times `-v` was used.

> Tip: With clap 3, it is possible to combine the builder and derive approaches. For example, [`clap::Args::augment_args_for_update`](https://docs.rs/clap/3/clap/trait.Args.html#tymethod.augment_args_for_update) can be used to flatten a derived list of arguments into a builder-based `App`.

## Command and argument case

Following Unix and GNU conventions, all commands and arguments, except for short arguments, *must* be in [kebab case](https://en.wikipedia.org/wiki/Kebab_case). This means that:

* Commands and arguments *must* be in lowercase.
* Multiple words *must* be separated by hyphens: `--example-opt`, not `--example_opt` or `--exampleOpt`.

`clap`'s derive feature and `structopt` use kebab case by default. If you have an existing command that doesn't follow these rules, you can maintain compatibility by renaming it to the kebab-cased version and retaining the old case as an alias.

## Alternatives to clap

* [argh](https://github.com/google/argh): Actively maintained, and has an explicit goal of being low-overhead at runtime. However, it [follows Fuchsia OS conventions](https://github.com/google/argh/issues/3#issuecomment-581144934) rather than Unix ones, so it's missing several crucial features from a Unix perspective.
* [pico-args](https://github.com/RazrFalcon/pico-args): Zero dependencies, quick to compile, and negligible impact on binary size. Does not include help generation, derive support, or as many config flags as clap. A great choice for really simple applications.
* [gumdrop](https://crates.io/crates/gumdrop): a simple argument parser with derive support. Somewhat less popular than clap, and doesn't support deserializing directly to domain types (clap [does](https://github.com/clap-rs/clap/blob/v3.0.6/examples/derive_ref/README.md#arg-types)).

For a comparison of binary size overhead and build times between these parsers and more, see [these benchmarks](https://github.com/rust-cli/argparse-benchmarks-rs).

## Writing your own parser by hand

You *should not* write your own parser completely by hand. Instead, most cases are better served by a simple parser like [pico-args](https://github.com/RazrFalcon/pico-args).

If you must write a parser by hand, consider basing it on the [lexopt](https://docs.rs/lexopt/latest/lexopt/) lexer. Be sure to handle all the [standard conventions for Unix CLIs](https://github.com/google/argh/issues/3#issuecomment-581144181).
