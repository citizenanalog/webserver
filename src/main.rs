use hyper::service::{make_service_fn, service_fn};
use hyper::{body::Bytes, Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::fs;

async fn handle(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Read index.html from the filesystem
    let contents = fs::read_to_string("/app/index.html")
        .unwrap_or_else(|_| String::from("Error: index.html not found"));
    Ok(Response::new(Body::from(contents)))
}

#[tokio::main]
async fn main() {
    // Construct our SocketAddr to listen on...
    let addr = SocketAddr::from(([0, 0, 0, 0], 80));

    // And a MakeService to handle each connection...
    let make_service = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    // Then bind and serve...
    let server = Server::bind(&addr).serve(make_service);

    // And run forever...
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
