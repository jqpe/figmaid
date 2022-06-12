use std::fs;

use serde::{Deserialize, Serialize};
use serde_json::{json, Map};
use ttf_parser::{fonts_in_collection, name_id, PlatformId};
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
pub struct Font {
    pub family: String,
    pub id: String,
    pub italic: bool,
    pub postscript: String,
    pub stretch: i8,
    pub style: String,
    pub weight: u16,
}

type FontDefinitions = Vec<Font>;

pub fn load_fonts(directories: Vec<String>) -> serde_json::Map<String, serde_json::Value> {
    let mut fonts = Map::new();
    let mut errors: Vec<walkdir::Error> = vec![];

    for dir in directories {
        for entry in WalkDir::new(dir) {
            match entry {
                Ok(entry) => {
                    let path = entry.path().to_str().unwrap();
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

    println!("{:?}", errors);

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

            for name in face.names() {
                match (name.name_id, name.platform_id) {
                    (name_id::FAMILY, PlatformId::Windows) => {
                        family = name.to_string()?;
                    }
                    (name_id::POST_SCRIPT_NAME, PlatformId::Windows) => {
                        postscript = name.to_string()?;
                    }
                    (name_id::SUBFAMILY, PlatformId::Windows) => {
                        style = name.to_string()?;
                    }
                    _ => {}
                };
            }

            fonts.push(Font {
                family: family.clone(),
                id: family,
                italic: face.is_italic(),
                postscript,
                stretch: 5,
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
