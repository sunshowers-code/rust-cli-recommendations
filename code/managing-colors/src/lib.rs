use std::fmt;
// ANCHOR: stylesheet
use owo_colors::{OwoColorize, Style};

// Stylesheet used to colorize MyValueDisplay below.
#[derive(Debug, Default)]
struct Styles {
    number_style: Style,
    shape_style: Style,
    // ... other styles
}

impl Styles {
    fn colorize(&mut self) {
        self.number_style = Style::new().bright_blue();
        self.shape_style = Style::new().bright_green();
        // ... other styles
    }
}
// ANCHOR_END: stylesheet

// ANCHOR: my_value
#[derive(Debug)]
pub struct MyValue {
    number: usize,
    shape: &'static str,
}

impl MyValue {
    pub fn new(number: usize, shape: &'static str) -> Self {
        Self { number, shape }
    }

    /// Returns a type that can display `MyValue`.
    pub fn display(&self) -> MyValueDisplay<'_> {
        MyValueDisplay {
            value: self,
            styles: Box::default(),
        }
    }
}

/// Displayer for [`MyValue`].
pub struct MyValueDisplay<'a> {
    value: &'a MyValue,
    styles: Box<Styles>,
}

impl<'a> MyValueDisplay<'a> {
    /// Colorizes the output.
    pub fn colorize(&mut self) {
        self.styles.colorize();
    }
}

impl<'a> fmt::Display for MyValueDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "My number is {}, and my shape is a {}",
            self.value.number.style(self.styles.number_style),
            self.value.shape.style(self.styles.shape_style),
        )
    }
}
// ANCHOR_END: my_value
