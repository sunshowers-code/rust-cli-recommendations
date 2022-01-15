// Run this binary with:
//    cd code
//    cargo run --bin grep-app -- <arguments>

// ANCHOR: definition
use clap::Parser;
use std::path::PathBuf;

/// A very simple utility to search for a string across multiple files.
#[derive(Debug, Parser)]
#[clap(name = "grep-app", version = "0.1.0", author = "My Awesome Team")]
pub struct GrepApp {
    /// Suppress normal output; instead print the name of each input file from which output
    /// would normally have been printed.  Scanning each input file stops upon first match.
    #[clap(long, short = 'l')]
    files_with_matches: bool,

    /// Search string
    search_str: String,

    /// Input files
    files: Vec<PathBuf>,
}
// ANCHOR_END: definition

fn main() {
    GrepApp::parse();
}

#[allow(dead_code)]
// ANCHOR: grep-help
const EXPECTED_HELP: &str = r#"grep-app 0.1.0
My Awesome Team
A very simple utility to search for a string across multiple files

USAGE:
    grep-app [OPTIONS] <SEARCH_STR> [FILES]...

ARGS:
    <SEARCH_STR>    Search string
    <FILES>...      Input files

OPTIONS:
    -h, --help                  Print help information
    -l, --files-with-matches    Suppress normal output; instead print the name of each input file
                                from which output would normally have been printed.  Scanning each
                                input file stops upon first match
    -V, --version               Print version information
"#;
// ANCHOR_END: grep-help

#[cfg(test)]
mod tests {
    use super::*;
    use clap::IntoApp;
    use std::io::Cursor;

    #[test]
    fn test_help() {
        let mut app = GrepApp::into_app();
        let mut cursor: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        app.write_help(&mut cursor).unwrap();
        let help = String::from_utf8(cursor.into_inner()).unwrap();
        println!("{}", help);
        assert_eq!(help, EXPECTED_HELP);
    }
}
