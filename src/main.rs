mod servers;

use servers::tcp::*;
use tokio::net::TcpStream;

async fn handle_stream(s: TcpStream) -> () {
    println!("-------------------- {:?}", s);
}

#[tokio::main]
async fn main() -> GenericResult<()> {
    tcp_init(handle_stream).await
}
