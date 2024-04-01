use colored::Colorize;
use clap::{
    Parser,
    Subcommand
};
use std::{
    io::{
        BufRead,
        BufReader,
        Write
    },
    net::{
        TcpListener,
        TcpStream,
        ToSocketAddrs
    },
    process,
    thread
};


/// Simple Chat Server
#[derive(Parser)]
#[command(about, version, long_about = None)]
pub struct Config {

    #[command(subcommand)]
    pub cmd: Cmd
}

#[derive(Subcommand)]
pub enum Cmd{
    Server {
        /// Specify IP to listen on, defaults to localhost
        #[arg(long, short, value_parser = validate_address)]
        address: Option<String>,

        /// Port to start the server on
        #[arg(long, short, default_value_t = 8080, value_parser = validate_port)]
        port: u16,

        /// Specifiy Identity
        #[arg(long, short)]
        identity: String,
    },

    Client {
        /// Specify remote server IP
        #[arg(long, short, value_parser = validate_address)]
        address: String,

        /// Specify remote server IP
        #[arg(long, short, default_value_t = 8080, value_parser = validate_port)]
        port: u16,

        /// Specifiy Identity
        #[arg(long, short)]
        identity: String,
    }
}

// port validation function
fn validate_port(port: &str) -> Result<u16, String> {
    let port = port
        .parse::<u16>()
        .map_err(|_| format!("{}", "[-] Not a port number!".red()))?;

    let valid_port_range = 1..=65535;

    if valid_port_range.contains(&port) {
        Ok(port)
    } else {
        Err(format!("{}", "[-] Not a valid port number!".red()))
    }
}

// Address validator function
fn validate_address(address: &str) -> Result<String, String> {
    let addr = format!("{}{}", address, ":8080");

    if addr.to_socket_addrs().is_err() {
        Err(format!("{}", "[-] Invalid address".red()))
    } else {
        Ok(address.to_string())
    }
}

impl Config {
    // Start client/server based on the type of the command
    pub fn start(self) {
        match self.cmd {
            Cmd::Server {address, port, identity} => {
                let listener = TcpListener::bind(
                    (
                        address.unwrap_or_else(|| String::from("127.0.0.1")),
                        port
                    )
                ).unwrap();

                for stream in listener.incoming() {
                    let stream_reader= stream
                        .unwrap();
                    println!("{} {:?}", "[+] Connection recieved from:".green(), stream_reader.peer_addr().unwrap());

                    start_threads(stream_reader, identity.clone());
                }
            },

            Cmd::Client {address, port, identity} => {
                let stream_reader = TcpStream::connect((address, port))
                    .unwrap_or_else(|e| {
                        println!("{} {:?}", "[-] Error connecting:".red(), e.kind());
                        process::exit(0);
                    });

                println!("{} {}", "[+] Connected to:".green(), stream_reader.peer_addr().unwrap());

                start_threads(stream_reader, identity);
            }
        }
    }
}

// Text reciever function
fn reciever(stream_reader: TcpStream) {
    let reader = BufReader::new(stream_reader);

    let mut reader_iter = reader
        .lines();

    let username = reader_iter
        .next()
        .unwrap()
        .unwrap();

    println!("{} {}", "You are talking to".green(), username.green());

    for lines in reader_iter {
        let username = format!("[{}]", username);
        println!("{} {}", username.blue(), lines.unwrap().blue());
    }
}

// Text sender function
fn sender(mut stream_writer: TcpStream) {
    loop {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)
            .unwrap();

        stream_writer.write_all(buffer.as_bytes())
            .unwrap();
    };
}

fn start_threads(stream: TcpStream, mut username: String) {
    let mut stream_writer = stream.try_clone()
        .unwrap();

    username.push_str("\n");
    stream_writer.write_all(username.as_bytes())
        .unwrap();

    let reader_handle = thread::spawn(move || {
        reciever(stream);
    });

    let writer_handle = thread::spawn(move || {
        sender(stream_writer);
    });

    reader_handle.join()
        .unwrap();
    writer_handle.join()
        .unwrap();
}
