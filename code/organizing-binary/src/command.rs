//! Contains command parsers and logic.

use clap::Parser;

#[derive(Debug, Parser)]
pub struct MyApp {
    // Options, subcommands etc
}

impl MyApp {
    pub fn exec(self) -> color_eyre::Result<()> {
        // execute the command and return a result
        unimplemented!()
    }
}