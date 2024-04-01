use clap::Parser;

fn main() {
    let config = server::Config::parse();
    config.start_listener();
}