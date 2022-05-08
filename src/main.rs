use hyper::Body;
use hyper::Request;
use hyper::Response;
use hyper::Server;
use std::convert::Infallible;
use std::net::SocketAddr;
use tower::make::Shared;
use tower::service_fn;
use tower::ServiceBuilder;

pub struct State {
    pub counter: i32,
}

async fn respond(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(format!("OK! State.counter = {}", 1).into()))
}

#[tokio::main]
async fn main() {
    // TODO: use --port from env
    // passing state to service
    let mut _state = State { counter: 1 };
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let service = service_fn(respond);

    let app = ServiceBuilder::new()
        // Add many layers as needed here
        .service(service);
    let server = Server::bind(&addr).serve(Shared::new(app));
    server.await.expect("Server Error");
}
