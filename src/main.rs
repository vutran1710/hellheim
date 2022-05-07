use hyper::Body;
use hyper::Request;
use hyper::Response;
use hyper::Server;
use std::convert::Infallible;
use std::net::SocketAddr;
use tower::make::Shared;
use tower::service_fn;
use tower::ServiceBuilder;

async fn hello_world(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World".into()))
}

#[tokio::main]
async fn main() {
    // TODO: use --port from env
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let service = ServiceBuilder::new().service(service_fn(hello_world));
    let server = Server::bind(&addr).serve(Shared::new(service));
    server.await.expect("Server Error");
}
