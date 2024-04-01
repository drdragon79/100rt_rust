use clap::Parser;
use std::net::TcpListener;
use std::{io, process};
use std::io::{
    BufRead,
    BufReader, Write
};

#[derive(Parser)]
#[command(about, version, long_about = None)]
pub struct Config {
    /// port to listen on
    #[arg(long, short, default_value_t = 8080, value_parser = port_check_range)]
    pub port: usize
}

fn port_check_range(param: &str) -> Result<usize, String> {
    let port = param
        .parse::<usize>()
        .map_err(|_| { format!("Not a port number")})?;
    let valid_port_range = 1..=65536;
    if valid_port_range.contains(&port) {
        Ok(port)
    } else {
        Err(format!("No a valid port number"))
    }
}

impl Config {
    pub fn start_server(&self) {
        let host = "127.0.0.1";
        let listener = TcpListener::bind((host, self.port as u16))
            .unwrap_or_else(|e| {
                println!("[-] Error occured: {}", e);
                process::exit(1);
            });
        for stream in listener.incoming() {
            let mut stream = stream
                .unwrap();
            println!("[+] Connected: {}", stream.peer_addr().unwrap());
            let mut stream_for_writing = stream.try_clone().unwrap();
            let reader = BufReader::new(&mut stream);
            for lines in reader.lines() {
                println!("[Client] {}", lines.unwrap());
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .unwrap();
                stream_for_writing.write_all(format!("[Server] {}", input).as_bytes())
                    .unwrap();
            }
        }
    }
}