use std::{fs, thread};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use tinyhttpserver::ThreadPool;


fn main() {
    let address = "::1:8887";
    //let address = "127.0.0.1:8887";
    let listener = TcpListener::bind(address).expect(&*format!("cannot bind to {}", address));
    println!("Running on {}\n", address);
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let template = |status:&str, template_file_as_string:String| {
        format!("{}Content-Length:{}\r\n\r\n{}",
                status, template_file_as_string.len(),
                template_file_as_string)};
    let mut buffer = [0u8; 1024];
    stream.read(&mut buffer).unwrap();
    let get_root = b"GET / HTTP/1.1\r\n";
    let get_sleep = b"GET /sleep HTTP/1.1\r\n";
    let html_ok = "HTTP/1.1 200 OK\r\n";
    let html_file_not_found = "HTTP/1.1 404 NOT FOUND\r\n";

    let (status_line, filename) = if buffer.starts_with(get_root) {
        (html_ok, "index.htm")
    } else if buffer.starts_with(get_sleep) {
        thread::sleep(Duration::from_secs(5));
        (html_ok, "zzz.htm")
    }
    else {
        (html_file_not_found, "404.htm")
    };
    let html_file = fs::read_to_string(filename).expect(&*format!("Failed to read template {}", filename));
    let response = template(status_line, html_file);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

