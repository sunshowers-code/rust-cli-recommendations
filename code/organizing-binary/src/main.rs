use clap::Parser;
use my_app::MyApp;

fn main() -> Result<(), ()> {
    let my_app = MyApp::parse();
    my_app.exec()
}
