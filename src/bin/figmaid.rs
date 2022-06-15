use figmaid::{cli::*, server::start_server};

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("config", config)) => {
            match config.subcommand() {
                Some(("create", _)) => {
                    create(false);
                }
                Some(("validate", _)) => {
                    validate().await;
                }
                Some(("open", _)) => open(),
                _ => {}
            };
        }
        Some((&_, _)) => {}
        None => {
            start_server().await;
        }
    }
}
