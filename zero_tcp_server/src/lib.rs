use clap::Parser;
use std::net::TcpListener;
use std::io::{
    BufRead,
    BufReader
};
use std::process;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// port to listen on
    #[arg(long, short, default_value_t = 8080, value_parser = port_within_range)]
    pub port: usize
}

fn port_within_range(s: &str) -> Result<usize, String>{
    let valid_port_range = 1..=65535;
    let port = s
        .parse::<usize>()
        .map_err(|_| format!("Not a port number"))?;
    if valid_port_range.contains(&port) {
        Ok(port)
    } else {
        Err(format!("Not a valid port number"))
    }
}

impl Config {
    pub fn start_listener(&self) {
        let host = "127.0.0.1";
        let listener = TcpListener::bind((host, self.port as u16))
            .unwrap_or_else(|e| {
                eprint!("[-] Error creating listener: {}", e);
                process::exit(1);
            });
        for stream in listener.incoming() {
            let mut stream = stream
                .unwrap();
            let peer_address = stream
                .peer_addr()
                .unwrap();
            println!("[+] Connection recieved from: {}", peer_address); 
            let reader = BufReader::new(&mut stream);
            for lines in reader.lines() {
                println!("{}", lines.unwrap());
            }
        }
    }
}