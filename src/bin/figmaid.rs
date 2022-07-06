use figmaid::{cli::*, config::load_config, server::start_server, service::font_metadata};

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
                _ => {
                    let config = load_config();

                    println!("port {} ", config.port);

                    for dir in &config.directories {
                        let fonts = font_metadata::load_fonts(vec![dir.to_string()]);

                        println!("{}: {} fonts", dir, fonts.len())
                    }
                }
            };
        }
        Some((&_, _)) => {}
        None => {
            start_server().await;
        }
    }
}
