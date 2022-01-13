# Handling arguments and subcommands

For a program that has subcommands, the following code structure is RECOMMENDED.

```rust,noplaypen
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
        #[clap(long, short)]
        opt: bool,

        path: PathBuf,
        // consider using camino's Utf8PathBuf instead:
        // https://docs.rs/camino/latest/camino/struct.Utf8PathBuf.html
    },
    /// Help message for write.
    Write {
        #[clap(flatten)]
        write_args: WriteArgs,
    }
}

#[derive(Debug, Parser)]
struct WriteArgs {
    path: PathBuf,
    // a list of other write args
}

#[derive(Debug, Parser)]
struct GlobalOpts {
    #[clap(long, arg_enum, global = true, default_value_t = Color::Auto)]
    color: Color,
    // ... other global options like -q/--quiet or -v/--verbose
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
  * This helps break up arguments into smaller components that can be more easily processed in different sections of the project's code. For example, `Color` can be further nested into an `OutputOpts` struct, defined in a separate `output.rs` file.
  * It also helps code pass a complex set of arguments around as a single parameter, rather than having to add a parameter everywhere.
* **Global options are marked with `#[clap(global = true)]`.**
  * This means that global options like `--color` can be used anywhere in the command line.
* **Use of `ArgEnum`.**
  * `ArgEnum` simplifies the definition of arguments that take one of a limited number of values.
