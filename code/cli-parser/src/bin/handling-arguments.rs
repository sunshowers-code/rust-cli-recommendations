// Run this binary with:
//    cd code
//    cargo run --bin grep-app -- <arguments>

// ANCHOR: definition
use camino::Utf8PathBuf;
use clap::{ArgEnum, Parser};

/// Here's my app!
#[derive(Debug, Parser)]
#[clap(name = "my-app", version)]
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
        #[clap(long, short = 'o')]
        example_opt: bool,

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
    #[clap(long, short, global = true, parse(from_occurrences))]
    verbose: usize,
    //... other global options
}

#[derive(Clone, Debug, ArgEnum)]
enum Color {
    Always,
    Auto,
    Never,
}
// ANCHOR_END: definition

fn main() {
    let app = App::parse();
    println!(
        "Verbosity level specified {} times",
        app.global_opts.verbose
    );
}

#[allow(dead_code)]
// ANCHOR: top-level-help
const EXPECTED_HELP: &str = r#"my-app 0.1.0
Here's my app!

USAGE:
    my-app [OPTIONS] <SUBCOMMAND>

OPTIONS:
        --color <COLOR>    Color [default: auto] [possible values: always, auto, never]
    -h, --help             Print help information
    -v, --verbose          Verbosity level (can be specified multiple times)
    -V, --version          Print version information

SUBCOMMANDS:
    help     Print this message or the help of the given subcommand(s)
    read     Help message for read
    write    Help message for write
"#;
// ANCHOR_END: top-level-help

#[allow(dead_code)]
// ANCHOR: read-help
const EXPECTED_READ_HELP: &str = r#"read 
Help message for read

USAGE:
    read [OPTIONS] <PATH>

ARGS:
    <PATH>    The path to read from

OPTIONS:
    -h, --help           Print help information
    -o, --example-opt    An example option
"#;
// ANCHOR_END: read-help

#[cfg(test)]
mod tests {
    use super::*;
    use clap::IntoApp;
    use std::io::Cursor;

    #[test]
    fn test_help() {
        let mut app = App::into_app();
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        app.write_help(&mut cursor).unwrap();
        let help = String::from_utf8(cursor.into_inner()).unwrap();
        println!("{}", help);
        assert_eq!(help, EXPECTED_HELP);
    }

    #[test]
    fn test_read_help() {
        let mut app = App::into_app();
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let read_cmd = app.find_subcommand_mut("read").unwrap();
        read_cmd.write_help(&mut cursor).unwrap();
        let help = String::from_utf8(cursor.into_inner()).unwrap();
        println!("{}", help);
        assert_eq!(help, EXPECTED_READ_HELP);
    }
}
