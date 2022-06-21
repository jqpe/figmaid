use clap::{crate_version, Command as Cmd};
use std::fs::File;
use std::io::{self, stdout, ErrorKind, Write};
use std::path::{Path, PathBuf};

use crate::config::{is_config_valid, Config};

pub fn cli() -> Cmd<'static> {
    Cmd::new("figmaid")
        .version(crate_version!())
        .about("Web server that allows you to use locally installed fonts in Figma")
        .subcommand(
            Cmd::new("config")
                .about("Create, open and validate configuration")
                .subcommand_required(true)
                .subcommand(Cmd::new("create").about("Create default configuration file"))
                .subcommand(Cmd::new("validate").about("Validate configuration"))
                .subcommand(Cmd::new("open").about("Open configuration file in text editor")),
        )
}

fn get_config_path() -> PathBuf {
    Path::new(&home::home_dir().expect("Couldn't get home directory"))
        .join(".config/figmaid/figmaid.json")
}

fn config_exists(config_path: &PathBuf) -> bool {
    File::open(&config_path).is_ok()
}

/// Validates the configuration file
pub async fn validate() {
    let config_path = get_config_path();

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
        Err(err) => {
            if err.kind() == ErrorKind::NotFound {
                eprintln!(
                    "Validation failed because the configuration file hasn't been created.\
                   \n-> run `figmaid config create` to create it."
                )
            } else {
                eprintln!("Couldn't validate because of io error: {}", err)
            }
        }
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
            Err(err) => {
                let home = home::home_dir().expect("Couldn't get home directory");

                let dotconfig = Path::join(&home, ".config");
                let figmaid = Path::join(&dotconfig, "figmaid");

                let panic_on_create_err = |path: &str| panic!("Couldn't create {:#?}", path);

                let (dotconfigerr, figmaiderr) = (
                    File::open(&dotconfig).is_err(),
                    File::open(&figmaid).is_err(),
                );

                if dotconfigerr {
                    std::fs::create_dir(&dotconfig)
                        .unwrap_or_else(|_| panic_on_create_err(dotconfig.to_str().unwrap()));
                }

                if figmaiderr {
                    std::fs::create_dir(&figmaid)
                        .unwrap_or_else(|_| panic_on_create_err(figmaid.to_str().unwrap()));
                }

                if figmaiderr || dotconfigerr {
                    return create(false);
                }

                eprintln!("Couldn't create configuration file: {:?}", err);
            }
        },
    }
}

/// Opens the configuration file
pub fn open() {
    let config_path = get_config_path();

    // Let's check that the configuration file exists before trying to open it
    if !config_exists(&config_path) {
        eprintln!("Couldn't open configuration file because it doesn't exist, create it with `figmaid config create`");
        return;
    }

    if let Err(err) = opener::open(config_path) {
        eprintln!("Couldn't open configuration file {}", err)
    }
}
