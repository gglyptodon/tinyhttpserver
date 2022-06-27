use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let address = "::1:8887";
    //let address = "127.0.0.1:8887";
    let listener = TcpListener::bind(address).expect(&*format!("cannot bind to {}", address));
    println!("Running on {}\n", address);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("connection request from: {}", &stream.peer_addr().unwrap());
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let template = |status:&str, template_file_as_string:String| {
        format!("{}Content-Length:{}\r\n\r\n{}",
                status, template_file_as_string.len(),
                template_file_as_string)};
    let mut buffer = [0u8; 1024];
    stream.read(&mut buffer).unwrap();
    println!("{:}", format!("{}", "*".repeat(30)));
    println!("Request: {} ", String::from_utf8_lossy(&buffer));
    println!("{:}", format!("{}", "*".repeat(30)));
    let get_root = b"GET / HTTP/1.1\r\n";

    let html_ok = "HTTP/1.1 200 OK\r\n";
    if buffer.starts_with(get_root) {
        let contents = fs::read_to_string("index.htm").expect("Failed to read html template");
        let response = format!(
            "{}Content-Length:{}\r\n\r\n{}",
            html_ok,
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let html_file_not_found = "HTTP/1.1 404 NOT FOUND\r\n";
        let html_file = fs::read_to_string("404.htm").expect("Failed to read 404 template");
        let response = template(html_file_not_found, html_file);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
