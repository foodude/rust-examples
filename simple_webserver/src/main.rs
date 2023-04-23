use clap::Parser;
use clap_num::number_range;
use std::fs;
use std::io::{prelude::*, BufReader};
use std::net::{IpAddr, TcpListener, TcpStream};
use std::thread;

fn main() {
    let args = Args::parse();

    let bind_address = format!("{}:{}", args.address.unwrap().to_string(), args.port);

    let listener = TcpListener::bind(bind_address).unwrap();

    match args.mode.as_str() {
        "single" => run_single_thread(listener),
        "multi" => run_multi_thread(listener),
        _ => eprintln!(
            "error, arg, [ -m | --mode ]: unknown argument < {} >",
            args.mode
        ),
    };
}

/// A simple webserver in Rust :)
#[derive(Parser, Debug)]
#[command(author, version)]
struct Args {
    /// IP address: IPv4, IPv6
    #[arg(short, long)]
    address: Option<IpAddr>,

    /// Port number: 1024 - 65535
    #[arg(short, long, value_parser=check_port_range)]
    port: u16,

    /// Server mode: single, multi
    #[arg(short, long)]
    mode: String,
}

fn check_port_range(port: &str) -> Result<u16, String> {
    number_range(port, 1024, 65535)
}

fn connection_handler(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, file_name) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(file_name).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
}

fn run_single_thread(listener: TcpListener) {
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        connection_handler(stream);
    }
}

fn run_multi_thread(listener: TcpListener) {
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            connection_handler(stream);
        });
    }
}
