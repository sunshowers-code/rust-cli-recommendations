#![allow(dead_code)]
use clap::{ArgEnum, Parser};
use my_app::MyValue;
use owo_colors::Stream;

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

// ANCHOR: stylesheet
// This example uses the supports-color crate:
// https://crates.io/crates/supports-color
//
// MyApp and Color definitions are repeated from the "immediate pattern"
// example above.

impl Color {
    fn supports_color_on(self, stream: Stream) -> bool {
        match self {
            Color::Always => true,
            Color::Auto => supports_color::on_cached(stream).is_some(),
            Color::Never => false,
        }
    }
}

fn main() {
    let app = MyApp::parse();

    let my_value = MyValue::new(24, "circle");
    let mut display = my_value.display();
    if app.color.supports_color_on(Stream::Stdout) {
        display.colorize();
    }
    println!("{}", display);
}
// ANCHOR_END: stylesheet
