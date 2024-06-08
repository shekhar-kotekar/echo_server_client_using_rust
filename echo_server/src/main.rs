use std::net::TcpListener;

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
    }
}
