use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let address = "::1:8887";
    //let address = "127.0.0.1:8887";
    let listener = TcpListener::bind(address).expect(&*format!("cannot bind to {}", address));
    println!("Running on {}\n", address);
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("connection request from: {}",&stream.peer_addr().unwrap());
        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0u8;1024];
    stream.read(&mut buffer).unwrap();
    println!("{:}", format!("{}","*".repeat(30)));
    println!("Request: {} ", String::from_utf8_lossy(&buffer));
    println!("{:}", format!("{}","*".repeat(30)));


    let html_ok = "HTTP/1.1 200 OK\r\n";
    //let response = "HTTP/1.1 418 I'm a teapot\r\n\r\n";
    let contents = fs::read_to_string("index.htm").expect("Failed to read html template");
    let response = format!("{}Content-Length:{}\r\n\r\n{}", html_ok, contents.len(), contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}