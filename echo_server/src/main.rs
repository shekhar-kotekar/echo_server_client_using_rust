use std::io::Read;
use std::io::Write;
use std::net::TcpListener;

use std::net::TcpStream;

const ECHO_SERVER_ADDRESS: &str = "127.0.0.1:5056";
fn main() {
    println!("Echo server listening on {}", ECHO_SERVER_ADDRESS);
    let listener = TcpListener::bind(ECHO_SERVER_ADDRESS).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!(
            "Connection accepted from {}:{}",
            stream.peer_addr().unwrap().ip(),
            stream.peer_addr().unwrap().port()
        );
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut message = [0; 1024];
    stream.read(&mut message).unwrap();
    stream.write_all(&message).unwrap();
    println!("echoed message: {}", String::from_utf8_lossy(&message));
}
