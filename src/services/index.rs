pub async fn get_index(
    state: actix_web::web::Data<crate::AppState>,
    req: actix_web::HttpRequest,
) -> Result<actix_web::HttpResponse, crate::server_error::ServerError> {
    let ctx = crate::template::template_context(&req);
    let rendered = state.tera.render("index.html", &ctx)?;
    return Ok(actix_web::HttpResponse::Ok().body(rendered));
}
