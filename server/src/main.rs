use std::path::Path;
use std::net::SocketAddr;

use futures::future;
use hyper::Server;
use hyper::service::{make_service_fn, service_fn};
use hyper_staticfile::Static;

mod routes;

const CLIENT: &str = "../client/";
const DEFAULT_ADDRESS: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() {
    let client = Static::new(Path::new(CLIENT));
    let cornish = make_service_fn(|_| {
        // to make a reference to client available inside of the `service_fn`
        let client = client.clone();
        future::ok::<_, hyper::Error>(service_fn(move |req| routes::handle(req, client.clone())))
    });

    let addr: SocketAddr = DEFAULT_ADDRESS
        .parse()
        .expect("Failed to parse address.");

    let server = Server::bind(&addr).serve(cornish);
    println!("Static files available at http://{}/", &addr);
    
    server.await
        .expect("Server failed");
}

