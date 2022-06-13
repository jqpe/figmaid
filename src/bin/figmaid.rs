use clap::Parser;
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;
use std::io::{self, Write};
use std::net::SocketAddr;

use figmaid::config::load_config;
use figmaid::service::figma;

/// Web server that allows you to use locally installed fonts in Figma
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Check configuration or create it if it doesn't exist
    #[clap(short)]
    t: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.t {
        let output = std::process::Command::new("figmaid-validate")
            .output()
            .expect("figmaid-validate is not installed");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        return;
    }

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
