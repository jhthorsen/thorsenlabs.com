use actix_web::HttpRequest;
use actix_web::http::header::HeaderValue;
use markdown::Markdown;
use rand::Rng;
use regex::Regex;
use serde_json::value::to_value;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;
use tera::{Context, Error, Tera};

pub mod markdown;

pub fn global_tera() -> Tera {
    static TERA: OnceLock<Tera> = OnceLock::new();
    TERA.get_or_init(|| {
        let mut tera = Tera::new(document_path("**/*.html").as_str()).unwrap();
        tera.register_filter("markdown", template_filter_markdown);
        tera.register_filter("match", template_filter_match);
        tera.register_function("markdown", template_function_markdown);
        tera.register_function("qs", template_function_qs);
        tera.register_function("slurp", template_function_slurp);
        tera
    })
    .to_owned()
}

fn header_value_to_string(value: Option<&HeaderValue>) -> String {
    value
        .unwrap_or(&HeaderValue::from_static(""))
        .to_str()
        .unwrap_or_default()
        .to_owned()
}

pub fn template_context(req: &HttpRequest) -> Context {
    let mut ctx = Context::new();
    let h = req.headers();
    let x_nonce = header_value_to_string(h.get("x-nonce"));

    let (csr, nonce) = match !x_nonce.is_empty() {
        true => (true, x_nonce),
        false => (
            false,
            rand::rng()
                .sample_iter(&rand::distr::Alphanumeric)
                .take(16)
                .map(char::from)
                .collect(),
        ),
    };

    ctx.insert("csr", &csr);
    ctx.insert("nonce", &nonce);
    ctx.insert("path", req.path());
    ctx.insert(
        "base_url",
        &env::var("THORSEN_BASE_URL").unwrap_or("https://thorsenlabs.com".to_owned()),
    );
    ctx
}

fn template_filter_match(
    value: &tera::Value,
    args: &HashMap<String, tera::Value>,
) -> Result<tera::Value, Error> {
    let re = Regex::new(args.get("re").unwrap().as_str().unwrap()).unwrap();
    Ok(to_value(re.is_match(value.as_str().unwrap()))?)
}

fn template_filter_markdown(
    value: &tera::Value,
    _args: &HashMap<String, tera::Value>,
) -> Result<tera::Value, Error> {
    let mut markdown = Markdown::new_from_path(&Path::new("/dev/null"));
    markdown.parse(serde_json::from_value::<String>(value.clone())?);
    Ok(to_value(markdown.content)?)
}

fn template_function_markdown(args: &HashMap<String, tera::Value>) -> Result<tera::Value, Error> {
    let mut path = String::new();
    if let Some(rel) = args.get("name") {
        path = document_path(rel.as_str().unwrap());
    }
    if path.len() > 0 {
        let mut markdown = Markdown::new_from_path(&Path::new(&path));
        if markdown.read() {
            return Ok(to_value(markdown.content)?);
        }
    }

    Err(format!("markdown path=\"{}\" could not be loaded", path).into())
}

fn template_function_qs(args: &HashMap<String, tera::Value>) -> Result<tera::Value, Error> {
    let mut path = String::new();

    if let Some(tera::Value::Object(orig)) = args.get("_") {
        for (key, value) in orig.iter() {
            if !args.contains_key(key) {
                if let Some(value) = value.as_str() {
                    path = format!(
                        "{}{}{}={}",
                        path,
                        if path.len() > 0 { "&" } else { "" },
                        key,
                        url_encode(value),
                    );
                }
            }
        }
    }

    for key in args.keys() {
        if let Some(tera::Value::String(value)) = args.get(key) {
            path = format!(
                "{}{}{}={}",
                path,
                if path.len() > 0 { "&" } else { "" },
                key,
                url_encode(value.as_str()),
            );
        }
    }

    return Ok(to_value(path)?);
}

fn template_function_slurp(args: &HashMap<String, tera::Value>) -> Result<tera::Value, Error> {
    let mut path = String::new();
    if let Some(rel) = args.get("name") {
        path = document_path(rel.as_str().unwrap());
        if let Ok(content) = fs::read_to_string(&path) {
            return Ok(to_value(content)?);
        }
        if let Some(fallback) = args.get("fallback") {
            return Ok(to_value(fallback)?);
        }
        return Ok(to_value("<!-- not found -->")?);
    }

    Err(format!("Raw path=\"{}\" could not be slurped", path).into())
}

pub fn document_path(rel: &str) -> String {
    format!(
        "{}/{}",
        env::var("THORSEN_DOCUMENT_DIR").unwrap_or("templates".to_string()),
        rel
    )
}

fn url_encode(str: &str) -> String {
    str.replace("%", "%25")
        .replace("?", "%3F")
        .replace("#", "%23")
}
