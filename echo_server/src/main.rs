use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

const ECHO_SERVER_ADDRESS: &str = "127.0.0.1:5056";

#[tokio::main]
async fn main() {
    start_tracing().await;
    tracing::info!("Echo server listening on {}", ECHO_SERVER_ADDRESS);
    let listener = TcpListener::bind(ECHO_SERVER_ADDRESS).await.unwrap();

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tracing::info!(
            "Connection accepted from {}:{}",
            stream.peer_addr().unwrap().ip(),
            stream.peer_addr().unwrap().port()
        );
        handle_connection(stream).await;
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut message = [0; 1024];
    stream.read(&mut message).await.unwrap();
    stream.write_all(&message).await.unwrap();
    tracing::info!("sent: {}", String::from_utf8_lossy(&message));
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
