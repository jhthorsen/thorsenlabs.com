use actix_web::http::header::HeaderValue;
use actix_web::HttpRequest;
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

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
    Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap()
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

    ctx
}
