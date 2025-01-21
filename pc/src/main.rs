use clap::Parser;
use pc::app::App;

fn main() {
    let app = App::parse();
    app.run().unwrap();
}
