use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const ECHO_SERVER_ADDRESS: &str = "localhost:5056";

#[tokio::main]
async fn main() {
    println!("Connecting to echo server at {}", ECHO_SERVER_ADDRESS);
    let result = TcpStream::connect(ECHO_SERVER_ADDRESS).await;
    match result {
        Ok(mut stream) => {
            println!(
                "Connected to echo server at {}:{}",
                stream.local_addr().unwrap().ip(),
                stream.local_addr().unwrap().port()
            );

            let message = format!("Hello from {}", stream.local_addr().unwrap().port());
            match stream.write_all(message.as_bytes()).await {
                Ok(_) => {
                    println!("Sent: {}", message);
                    let mut buffer: String = String::new();
                    stream.read_to_string(&mut buffer).await.unwrap();
                    println!("Received: {}", buffer);
                }
                Err(e) => {
                    println!("Error sending message: {}", e);
                }
            };
        }
        Err(e) => println!("Error connecting to echo server: {}", e),
    }
}
