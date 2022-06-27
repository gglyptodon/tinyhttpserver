use std::net::TcpListener;
fn main() {
    let address = "127.0.0.1:8887";
    let listener = TcpListener::bind(address).expect(&*format!("cannot bind to {}", address));
    println!("Running on {}", address);
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        println!("connection established")
    }

}
