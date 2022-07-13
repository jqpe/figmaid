//! Handles requests to figma routes.
//! - GET|OPTIONS `/figma/font-files` for a JSON file of all loaded fonts.
//! - GET|OPTIONS `/figma/font-file` for a single font file.
//!
//! OPTIONS is provided for CORS preflight requests with adequate response headers.
//!
//! Attempt to access other routes is considered an error from the user (should be Figma)
//! and Bad Request is sent as a status code with empty body.

use hyper::{body::Bytes, header::HeaderValue, Body, Method, Request, Response, StatusCode};
use regex::Regex;
use serde_json::json;
use std::{convert::Infallible, fs};
use url::Url;

use crate::font_metadata::load_fonts;

use crate::config::{load_config, Config};

const FIGMA_FONT_FILES: &str = "/figma/font-files";
const FIGMA_FONT_FILE: &str = "/figma/font-file";

pub async fn figma(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let config = load_config();

    let mut response = Response::builder()
        .header("access-control-allow-private-network", "true")
        .header("access-control-allow-origin", "https://www.figma.com")
        .body(Body::empty())
        .unwrap();

    match (req.uri().path(), req.method()) {
        (FIGMA_FONT_FILES, &Method::GET | &Method::OPTIONS) => {
            handle_font_files(config, &mut response)
        }
        (FIGMA_FONT_FILE, &Method::GET | &Method::OPTIONS) => handle_font_file(req, &mut response),
        _ => {
            *response.status_mut() = StatusCode::BAD_REQUEST;
        }
    };

    Ok(response)
}

/// Responds with a single font file with headers for otf/ttf/ttc
fn handle_font_file(req: Request<Body>, response: &mut Response<Body>) {
    if let Ok(url) = Url::parse(
        ["http://localhost", req.uri().to_string().as_str()]
            .join("")
            .as_str(),
    ) {
        let file = url.query_pairs().find(|(key, _)| key == "file");

        if let Some((_, file)) = file {
            if let Ok(font_file) = fs::read(&*file) {
                let re = Regex::new(r"(otf|ttc|ttf)$").unwrap();
                if let Some(ext) = re.find(&file) {
                    match ext.as_str() {
                        "ttf" | "ttc" => {
                            response.headers_mut().insert(
                                "content-type",
                                HeaderValue::from_static("application/x-font-ttf"),
                            );
                        }
                        "otf" => {
                            response
                                .headers_mut()
                                .insert("content-type", HeaderValue::from_static("font/otf"));
                        }
                        _ => {}
                    };
                };

                *response.body_mut() = Body::from(Bytes::from(font_file));
            }
        }
    } else {
        *response.status_mut() = StatusCode::NOT_FOUND;
    }
}

/// Responds with a JSON of all font files
fn handle_font_files(config: Config, response: &mut Response<Body>) {
    let font_files: serde_json::Value = load_fonts(config.directories).into();
    let json = json!({
        "version": 20i8,
        "fontFiles": font_files,
    });
    response
        .headers_mut()
        .insert("content-type", HeaderValue::from_static("application/json"));
    *response.body_mut() = Body::from(json.to_string());
}
