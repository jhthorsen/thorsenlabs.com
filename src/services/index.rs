use actix_web::{http::header::ContentType, HttpResponse};

pub async fn get_index(
    state: actix_web::web::Data<crate::AppState>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, crate::server_error::ServerError> {
    let ctx = crate::template::template_context(&req);
    let rendered = state.tera.render("index.html", &ctx)?;
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(rendered))
}
