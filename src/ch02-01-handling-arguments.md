# Handling arguments and subcommands

For a program that has subcommands, the following code structure is RECOMMENDED.

```rust,noplaypen
use camino::Utf8PathBuf;
use clap::{ArgEnum, Parser};

/// Help message for the app.
#[derive(Debug, Parser)]
pub struct App {
    #[clap(flatten)]
    global_opts: GlobalOpts,

    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    /// Help message for read.
    Read {
        /// An example option
        #[clap(long, short)]
        opt: bool,

        /// The path to read from
        path: Utf8PathBuf,
    },
    /// Help message for write.
    Write {
        #[clap(flatten)]
        write_args: WriteArgs,
    },
    // ...other commands (can also #[clap(flatten)] other enums here)
}

#[derive(Debug, Parser)]
struct WriteArgs {
    /// The path to write to
    path: Utf8PathBuf,
    // a list of other write args
}

#[derive(Debug, Parser)]
struct GlobalOpts {
    /// Color
    #[clap(long, arg_enum, global = true, default_value_t = Color::Auto)]
    color: Color,

    /// Verbosity level (can be specified multiple times)
    #[clap(long, short, global = true, multiple_occurrences = true)]
    verbose: u64,

    //... other global options
}

#[derive(Clone, Debug, ArgEnum)]
enum Color {
    Always,
    Auto,
    Never,
}
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
