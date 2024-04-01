use clap::Parser;

fn main() {
    let config = chat::Config::parse();
    config.start();
}
