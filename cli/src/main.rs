use clap::Parser;
use cli::app::App;

fn main() {
    let app = App::parse();
    app.run().unwrap();
}
