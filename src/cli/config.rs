use std::fs::File;
use std::io::{self, stdout, ErrorKind, Write};
use std::path::{Path, PathBuf};

use crate::config::{is_config_valid, Config};

fn get_config_path() -> PathBuf {
    Path::new(&home::home_dir().expect("Couldn't get home directory"))
        .join(".config/figmaid/figmaid.json")
}

fn config_exists(config_path: &PathBuf) -> bool {
    File::open(config_path).is_ok()
}

/// Validates the configuration file
pub async fn validate() {
    let config_path = get_config_path();

    match std::fs::read(config_path) {
        Ok(config) => {
            let config = serde_json::from_slice(&config);

            match (config.is_ok(), is_config_valid(&config.unwrap()).await) {
                (true, true) => println!("Configuration is OK."),

                (true, false) => eprintln!("Configuration is not valid."),

                (false, _) => eprintln!("Configuration is not valid JSON."),
            }
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => eprintln!(
                "Validation failed because the configuration file hasn't been created.\
                   \n-> run `figmaid config create` to create it."
            ),
            _ => eprintln!("Couldn't validate because of io error: {}", e),
        },
    }
}

/// Creates the configuration file
///
/// # Arguments
/// * `force` Overwite exisiting configuration file
pub fn create(force: bool) {
    let config_path = get_config_path();

    match (config_exists(&config_path), force) {
        (true, false) => {
            print!("Configuration file exists, overwrite with the default configuration? y/N ");

            stdout().flush().expect("Couldn't flush stdout");

            let mut choice = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("Couldn't read from stdin");

            match choice.trim() {
                "y" => create(true),
                _ => {
                    println!("Ok, skipping!");
                }
            }
        }
        _ => match std::fs::write(config_path, Config::default().to_string()) {
            Ok(_) => {
                println!(
                    "Succesfully {} configuration file.",
                    match force {
                        true => "overwrote",
                        false => "created",
                    }
                );
            }
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    match create_missing_dirs() {
                        Ok(_) => create(false),
                        Err(_) => eprintln!("Couldn't create configuration file: {:?}", e),
                    }
                };
            }
        },
    }
}

/// Create ~/.config and ~/.config/figmaid if they don't exist
fn create_missing_dirs() -> io::Result<()> {
    let home = home::home_dir().expect("Couldn't get home directory");

    let figmaid = Path::join(&home, ".config/figmaid");

    std::fs::create_dir_all(&figmaid)
}

/// Opens the configuration file
pub fn open() {
    let config_path = get_config_path();

    // Let's check that the configuration file exists before trying to open it
    if !config_exists(&config_path) {
        eprintln!("Couldn't open configuration file because it doesn't exist, create it with `figmaid config create`");
        return;
    }

    if let Err(e) = opener::open(config_path) {
        eprintln!("Couldn't open configuration file {}", e)
    }
}
