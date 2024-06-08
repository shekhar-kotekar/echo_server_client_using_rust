use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const ECHO_SERVER_ADDRESS: &str = "localhost:5056";

#[tokio::main]
async fn main() {
    start_tracing().await;
    tracing::info!("Connecting to echo server at {}", ECHO_SERVER_ADDRESS);
    let result = TcpStream::connect(ECHO_SERVER_ADDRESS).await;
    match result {
        Ok(mut stream) => {
            tracing::info!(
                "Connected to echo server at {}:{}",
                stream.local_addr().unwrap().ip(),
                stream.local_addr().unwrap().port()
            );

            let message = format!("Hello from {}", stream.local_addr().unwrap().port());
            match stream.write_all(message.as_bytes()).await {
                Ok(_) => {
                    tracing::info!("Sent: {}", message);
                    let mut buffer: String = String::new();
                    stream.read_to_string(&mut buffer).await.unwrap();
                    tracing::info!("Received: {}", buffer);
                }
                Err(e) => {
                    tracing::error!("Error sending message: {}", e);
                }
            };
        }
        Err(e) => tracing::error!("Error connecting to echo server: {}", e),
    }
}

async fn start_tracing() {
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    tracing::info!("Tracing started");
}
