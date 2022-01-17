use clap::Parser;
use my_app::MyApp;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let my_app = MyApp::parse();
    my_app.exec()
}
