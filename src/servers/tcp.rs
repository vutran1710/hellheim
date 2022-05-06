use futures::future::BoxFuture;
use std::error::Error;
use tokio::net::TcpListener;
use tokio::net::TcpStream;

type GenericResult<T> = Result<T, Box<dyn Error>>;

pub async fn tcp_init() -> GenericResult<()> {
    let addr = "127.0.0.1:8080".to_string();
    let server = TcpListener::bind(&addr).await?;

    println!("-------------------- Listening on: {}", addr);

    loop {
        let (stream, _) = server.accept().await?;
        tokio::spawn(async move {
            // Handling the stream
            // passing the handler-chain as argument
        });
    }
}
