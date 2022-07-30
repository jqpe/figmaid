use figmaid::font_metadata;
use ttf_parser::Width;

use serde_json::*;

#[test]
fn extract_font_metadata() {
    let font = include_bytes!("data/impact.ttf");

    let data = font_metadata::extract_font_metadata(font.to_vec()).expect("can read the font data");
    let obj = data[0].as_object().unwrap();

    assert_eq!(obj.as_str("family"), "Impact");

    assert_eq!(obj.as_str("id"), "Impact - 1992");

    assert!(!obj.as_bool("italic"));

    assert_eq!(obj.as_str("postscript"), "Impact");

    assert_eq!(obj.as_i64("stretch"), Width::Condensed.to_number() as i64);

    assert_eq!(obj.as_str("style"), "Normal");

    assert_eq!(obj.as_i64("weight"), 400);
}

trait Test {
    fn as_str(&self, key: &str) -> &str;
    fn as_i64(&self, key: &str) -> i64;
    fn as_bool(&self, key: &str) -> bool;
}

impl Test for Map<String, Value> {
    fn as_str(&self, key: &str) -> &str {
        self.get(key)
            .unwrap_or_else(|| panic!("expected object to have ${key}"))
            .as_str()
            .unwrap_or_else(|| panic!("expected ${key} to be a string"))
    }

    fn as_i64(&self, key: &str) -> i64 {
        self.get(key)
            .unwrap_or_else(|| panic!("expected object to have ${key}"))
            .as_i64()
            .unwrap_or_else(|| panic!("expected ${key} to be a i64"))
    }

    fn as_bool(&self, key: &str) -> bool {
        self.get(key)
            .unwrap_or_else(|| panic!("expected object to have ${key}"))
            .as_bool()
            .unwrap_or_else(|| panic!("expected ${key} to be a bool"))
    }
}
