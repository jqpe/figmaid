use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;
use std::net::SocketAddr;

use figmaid::config::load_config;
use figmaid::service::figma;

#[tokio::main]
async fn main() {
    let config = load_config();
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    let figma = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(figma)) });
    let server = Server::try_bind(&addr);

    match server {
        Ok(server) => {
            println!("Server started on http://{}", &addr);

            if let Err(e) = server.serve(figma).await {
                eprintln!("server error: {}", e);
            }
        }
        Err(err) => panic!("{:?}", err),
    }
}
