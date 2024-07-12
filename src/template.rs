use actix_web::http::header::HeaderValue;
use actix_web::HttpRequest;
use markdown::Markdown;
use serde::{Deserialize, Serialize};
use serde_json::value::to_value;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::sync::OnceLock;
use tera::{Context, Error, Tera, Value};

pub mod markdown;

#[derive(Serialize, Deserialize, Debug)]
struct HtmxHeaders {
    boosted: bool,
    history_restore_request: bool,
    request: bool,
    target: String,
    trigger: String,
    trigger_name: String,
}

pub fn global_tera () -> Tera {
    static TERA: OnceLock<Tera> = OnceLock::new();
    TERA.get_or_init(|| {
        if env::var_os("THORSEN_DOCUMENT_DIR").is_none() {
            env::set_var("THORSEN_DOCUMENT_DIR", "templates");
        }

        let mut tera = Tera::new(document_path("**/*.html").as_str()).unwrap();
        tera.register_filter("markdown", template_filter_markdown);
        tera.register_function("markdown", template_function_markdown);
        tera
    }).to_owned()
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
    ctx.insert("base_url", &env::var("THORSEN_BASE_URL").unwrap_or("https://thorsen.pm".to_owned()));
    ctx
}

fn template_filter_markdown(value: &Value, _args: &HashMap<String, Value>) -> Result<Value, Error> {
    let mut markdown = Markdown::new_from_path(&Path::new("/dev/null"));
    markdown.parse(serde_json::from_value::<String>(value.clone())?);
    Ok(to_value(markdown.content)?)
}

fn template_function_markdown(args: &HashMap<String, Value>) -> Result<Value, Error> {
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

pub fn document_path(rel: &str) -> String {
    format!("{}/{}", env::var("THORSEN_DOCUMENT_DIR").unwrap(), rel)
}
