use std::fs;

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map};
use ttf_parser::{fonts_in_collection, name_id};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
pub struct Font {
    pub family: String,
    pub id: String,
    pub italic: bool,
    pub postscript: String,
    pub stretch: u16,
    pub style: String,
    pub weight: u16,
}

type FontDefinitions = Vec<Font>;

pub fn load_fonts(directories: Vec<String>) -> serde_json::Map<String, serde_json::Value> {
    let mut fonts = Map::new();
    let mut errors: Vec<walkdir::Error> = vec![];

    let re = Regex::new(r"\.(otf|ttf|otc|ttc)$").unwrap();

    for dir in directories {
        for entry in WalkDir::new(dir) {
            match entry {
                Ok(entry) => {
                    let path = entry.path().to_str().unwrap();

                    if !re.is_match(path) {
                        continue;
                    }

                    let data = fs::read(path);

                    if let Ok(data) = data {
                        if let Some(json) = extract_font_metadata(data) {
                            fonts.insert(path.to_string(), json);
                        }
                    }
                }
                Err(error) => errors.push(error),
            }
        }
    }

    if !(errors.is_empty()) {
        eprintln!("{:?}", errors);
    }

    fonts
}

pub fn extract_font_metadata(data: Vec<u8>) -> Option<serde_json::Value> {
    let mut fonts: FontDefinitions = vec![];
    let collection_items = fonts_in_collection(&data).unwrap_or(1);

    for i in 0..collection_items {
        if let Ok(face) = ttf_parser::Face::from_slice(&data, i) {
            let mut family = String::new();
            let mut postscript = String::new();
            let mut style = String::new();
            let mut id = String::new();

            for name in face.names().into_iter().filter(|name| name.is_unicode()) {
                match name.name_id {
                    name_id::FAMILY | name_id::TYPOGRAPHIC_FAMILY => {
                        family = name.to_string()?;
                        style = name.to_string()?;
                    }
                    name_id::POST_SCRIPT_NAME => {
                        postscript = name.to_string()?;
                    }
                    name_id::SUBFAMILY | name_id::TYPOGRAPHIC_SUBFAMILY => {
                        style = name.to_string()?;
                    }
                    name_id::UNIQUE_ID => {
                        id = name.to_string()?;
                    }
                    _ => {}
                };
            }

            // This might happen with .woff files.
            if family == "false" {
                continue;
            }

            fonts.push(Font {
                family,
                id,
                italic: face.is_italic(),
                postscript,
                stretch: face.width().to_number(),
                style,
                weight: face.weight().to_number(),
            })
        }
    }

    if !fonts.is_empty() {
        return Some(json!(fonts));
    }

    None
}
