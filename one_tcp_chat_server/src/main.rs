use clap::Parser;

fn main() {
    let config = chat_server::Config::parse();
    config.start_server();
}
