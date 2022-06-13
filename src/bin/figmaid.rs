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
        return validate().await;
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

/// Validates the configuration file and optionally creates the default configuration if it doesn't exist.
async fn validate() {
    use std::{io::stdout, path::Path};

    use figmaid::config::{is_config_valid, Config};
    use opener::open;

    let config_path = Path::new(&home::home_dir().expect("Couldn't get home directory"))
        .join(".config/figmaid/figmaid.json");

    match std::fs::read(config_path.clone()) {
        Ok(config) => {
            if let Ok(config) = serde_json::from_str(&String::from_utf8_lossy(&config)) {
                if is_config_valid(&config).await {
                    println!("Configuration is OK.")
                }
            } else {
                println!("Configuration is not valid JSON.")
            }
        }
        _ => {
            let mut choice = String::new();

            eprint!("Couldn't load configuration file, create it y/N? ");
            stdout()
                .flush()
                .unwrap_or_else(|err| eprintln!("Couldn't flush stdout: {err}"));

            io::stdin()
                .read_line(&mut choice)
                .expect("Couldn't read line");

            match choice.trim() {
                "y" | "Y" => {
                    match std::fs::write(config_path.clone(), Config::default().to_string()) {
                        Ok(_) => {
                            print!("Succesfully created configuration file, open it y/N? ");
                            stdout()
                                .flush()
                                .unwrap_or_else(|err| eprintln!("Couldn't flush stdout: {err}"));

                            let mut choice = String::new();

                            io::stdin()
                                .read_line(&mut choice)
                                .expect("Couldn't read line");

                            if "y" == choice.trim() {
                                open(config_path).expect("Couldn't open");
                            }
                        }
                        Err(err) => {
                            eprintln!("Couldn't create configuration file: {}", err)
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
