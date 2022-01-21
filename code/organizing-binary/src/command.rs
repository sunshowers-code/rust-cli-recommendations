//! Contains command parsers and logic.

use clap::Parser;

#[derive(Debug, Parser)]
pub struct MyApp {
    // Options, subcommands etc
    #[clap(short, long, default_value_t)]
    my_arg: usize,
}

impl MyApp {
    pub fn exec(self) -> color_eyre::Result<()> {
        println!("The value of my-arg is {}", self.my_arg);
        Ok(())
    }
}
