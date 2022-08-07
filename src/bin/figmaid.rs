use figmaid::{cli::cli, cli::config, config::load_config, font_metadata, server::start_server};

#[tokio::main]
async fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("config", config)) => {
            match config.subcommand() {
                Some(("create", _)) => {
                    config::create(false);
                }
                Some(("validate", _)) => {
                    config::validate().await;
                }
                Some(("open", _)) => config::open(),
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
