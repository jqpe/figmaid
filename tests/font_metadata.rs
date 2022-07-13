use figmaid::font_metadata;
use ttf_parser::Width;

#[test]
fn extract_font_metadata() {
    let font = include_bytes!("data/impact.ttf");

    let data = font_metadata::extract_font_metadata(font.to_vec()).expect("can read the font data");
    let obj = data[0]
        .as_object()
        .expect("it's an array of font definitions in json format");

    // family
    let family = obj
        .get("family")
        .expect("has family")
        .as_str()
        .expect("family is a string");

    assert_eq!(family, "Impact");

    // id
    let id = obj
        .get("id")
        .expect("has id")
        .as_str()
        .expect("id is a string");

    assert_eq!(id, "Impact - 1992");

    // italic
    let italic = obj
        .get("italic")
        .expect("has italic")
        .as_bool()
        .expect("italic is a boolean");

    assert!(!italic);

    // postscript
    let postscript = obj
        .get("postscript")
        .expect("has postscript")
        .as_str()
        .expect("postscript is a string");

    assert_eq!(postscript, "Impact");

    // stretch
    let stretch = obj
        .get("stretch")
        .expect("has stretch")
        .as_i64()
        .expect("stretch is a number");

    assert_eq!(stretch, Width::Condensed.to_number() as i64);

    // style
    let style = obj
        .get("style")
        .expect("has style")
        .as_str()
        .expect("style is a string");

    assert_eq!(style, "Normal");

    // weight
    let weight = obj
        .get("weight")
        .expect("has weight")
        .as_i64()
        .expect("weight is an integer");

    assert_eq!(weight, 400);
}
