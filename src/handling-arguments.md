# Handling arguments and subcommands

For a program that has subcommands, the following code structure is *recommended*.

```rust,noplaypen
{{#rustdoc_include ../code/cli-parser/src/bin/handling-arguments.rs:definition}}
```

Notes:
* **Only the top-level `App` is public.**
* **`App` is a struct, one level above the command enum.**
  * While it is possible to make `App` an enum with all the subcommands, in my experience this design has always come back to bite me. This has always been because I've wanted to introduce global options later.
* **Liberal use of `#[clap(flatten)]`.**
  * This option flattens inline options from a struct into the parent struct or enum variant, or from an enum into a parent enum.
  * This helps break up long series of options into smaller, reusable components that can be more easily processed in different sections of the project's code. For example, `Color` can be further nested into an `OutputOpts` struct, defined in a separate `output.rs` file.
  * It also helps code pass a complex set of arguments around as a single parameter, rather than having to add a parameter everywhere.
* **Global options are marked with `#[clap(global = true)]`.**
  * This means that global options like `--color` can be used anywhere in the command line.
* **Use of `ArgEnum`.**
  * `ArgEnum` simplifies the definition of arguments that take one of a limited number of values.

---

The top-level help message is:

```rust,noplaypen
{{#rustdoc_include ../code/cli-parser/src/bin/handling-arguments.rs:top-level-help}}
```

The help for the read command is:

```rust,noplaypen
{{#rustdoc_include ../code/cli-parser/src/bin/handling-arguments.rs:read-help}}
```
