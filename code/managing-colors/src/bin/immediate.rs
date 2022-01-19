#![allow(dead_code)]
// ANCHOR: immediate
// The owo-colors "supports-colors" feature must be enabled.
use clap::{ArgEnum, Parser};
use owo_colors::{OwoColorize, Stream};

#[derive(Debug, Parser)]
struct MyApp {
    #[clap(long, arg_enum, global = true, default_value = "auto")]
    color: Color,
}

#[derive(ArgEnum, Clone, Copy, Debug)]
enum Color {
    Always,
    Auto,
    Never,
}

impl Color {
    fn init(self) {
        // Set a supports-color override based on the variable passed in.
        match self {
            Color::Always => owo_colors::set_override(true),
            Color::Auto => {}
            Color::Never => owo_colors::set_override(false),
        }
    }
}

fn main() {
    let app = MyApp::parse();
    app.color.init();

    println!(
        "My number is {}",
        42.if_supports_color(Stream::Stdout, |text| text.bright_blue())
    );
}
// ANCHOR_END: immediate
