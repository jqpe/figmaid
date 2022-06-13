//! Provides utilies for validating and loading the configuration file in addition to providing a default implementation.

use home::home_dir;
use jsonschema::JSONSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, fs, path::Path};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub directories: Vec<String>,
}

impl From<&Config> for serde_json::Value {
    fn from(config: &Config) -> Self {
        json!(config)
    }
}

impl Config {
    pub fn new(port: u16, directories: Vec<String>) -> Self {
        Self {
            port: match env::var("PORT") {
                Ok(port) => port.parse().unwrap_or_else(|err| {
                    eprintln!(
                        "PORT environment variable was set, but couldn't be parsed into an unsigned integer: {}",
                        err
                    );
                    18412
                }),
                _ => port,
            },
            directories: match env::var("DIRS") {
                Ok(dirs) => {
                    let dirs = dirs
                        .split(',')
                        .map(|str| str.to_string())
                        .collect::<Vec<String>>();

                    dirs
                }
                _ => directories,
            },
         }
    }
}

impl ToString for Config {
    fn to_string(&self) -> String {
        if let Ok(pretty) = serde_json::to_string_pretty(&self) {
            pretty
        } else {
            String::from("")
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new(
            18412,
            vec![
                "/usr/share/fonts".to_string(),
                "/usr/lib/share/fonts".to_string(),
            ],
        )
    }
}

/// Load configuration file from `~/.config/figmaid/figmaid.json` or fallback to the default configuration.
///
/// The ~ represents the current user's home directory.
///
/// Environment variables have preference over configuration.
///
/// If the environment variables could not be parsed (e.g. passing '-19' into PORT) the configuration value (or default) will be used.
pub fn load_config() -> Config {
    let config_path = Path::new(&home_dir().unwrap()).join(".config/figmaid/figmaid.json");

    let default_config = Config::default();

    if let Ok(config) = fs::read(config_path) {
        let json_string = String::from_utf8_lossy(&config);
        let json = serde_json::from_str(&json_string).unwrap_or(default_config);

        return Config::new(json.port, json.directories);
    }

    default_config
}

/// Attempts to load the schema from ./docs/schema.json.
pub fn load_schema_json() -> Option<serde_json::Value> {
    let schema = fs::read(Path::new("./docs/schema.json"));

    if let Ok(schema) = schema {
        if let Ok(schema) = String::from_utf8(schema) {
            let schema: Result<serde_json::Value, _> = serde_json::from_str(schema.as_str());

            if let Ok(schema) = schema {
                return Some(schema);
            }
        }
    }

    None
}

/// Validates the configuration. Logs any errors to stderr.
pub fn is_config_valid(json: &serde_json::Value) -> bool {
    match load_schema_json() {
        Some(schema) => {
            let compiled = JSONSchema::compile(&schema).expect("A valid schema");
            let validation = compiled.validate(json);

            if let Err(errors) = validation {
                for error in errors {
                    eprintln!("Validation error: {} {}", error.instance_path, error);
                }

                return false;
            }

            true
        }
        None => {
            eprintln!("Couldn't load schema.");
            false
        }
    }
}
