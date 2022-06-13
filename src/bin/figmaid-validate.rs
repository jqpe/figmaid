use std::{
    io::{self, stdout, Write},
    path::Path,
};

use figmaid::config::{is_config_valid, Config};
use opener::open;

#[tokio::main]
async fn main() {
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
