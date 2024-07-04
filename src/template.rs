use actix_web::HttpRequest;
use actix_web::http::header::HeaderValue;
use pulldown_cmark;
use serde::{Deserialize, Serialize};
use serde_json::value::to_value;
use std::collections::HashMap;
use std::fs;
use tera::{Context, Error, Tera, Value};

#[derive(Serialize, Deserialize, Debug)]
struct HtmxHeaders {
    boosted: bool,
    history_restore_request: bool,
    request: bool,
    target: String,
    trigger: String,
    trigger_name: String,
}

pub fn build_tera() -> Tera {
    let mut tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    tera.register_filter("markdown", template_filter_markdown);
    tera.register_function("markdown", template_function_markdown);
    tera
}

fn header_value_to_string(value: Option<&HeaderValue>) -> String {
    value
        .unwrap_or(&HeaderValue::from_static(""))
        .to_str()
        .unwrap_or_default()
        .to_string()
}

pub fn template_context(req: HttpRequest) -> Context {
    let mut ctx = Context::new();
    let h = req.headers();
    ctx.insert(
        "htmx",
        &HtmxHeaders {
            boosted: h.get("HX-Boosted").is_some(),
            history_restore_request: h.get("HX-History-Restore-Request").is_some(),
            request: h.get("HX-Request").is_some(),
            target: header_value_to_string(h.get("HX-Target")),
            trigger: header_value_to_string(h.get("HX-Trigger")),
            trigger_name: header_value_to_string(h.get("HX-Trigger-Name")),
        },
    );

    ctx.insert("path", req.path());

    ctx
}

fn template_filter_markdown(
    value: &Value,
    _args: &HashMap<String, Value>,
) -> Result<Value, Error> {
    let text = serde_json::from_value::<String>(value.clone())?;
    let parser = pulldown_cmark::Parser::new(&text);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    Ok(to_value(html_output)?)
}

fn template_function_markdown(args: &HashMap<String, Value>) -> Result<Value, Error> {
    match args.get("file") {
        Some(file) => {
            let mut file = file.as_str().unwrap_or("index").trim_end_matches('/');
            if file.len() <= 1 {
                file = "index";
            }

            let mut path = format!("templates/{}.md", file);
            let path_metadata = fs::metadata(path.clone())?;
            if path_metadata.is_dir() {
                path = format!("{}/index.md", path);
            }

            let contents = fs::read_to_string(path.clone());
            match contents {
                Ok(contents) => template_filter_markdown(&Value::String(contents), args),
                Err(err) => Ok(Value::String(format!("Could not open {}: {}", path, err))),
            }
        },
        None => Ok(Value::String(format!("Missing file argument"))),
    }
}
