use std::sync::OnceLock;
use actix_web::web::get;
use super::models::http::{
    Bytes,
    ContentType,
    HttpResponse,
    Responder,
    ServiceConfig
};
use crate::{
    config,
    utils::string::process_template
};

static INDEX_HTML: OnceLock<String> = OnceLock::new();

pub fn configure_index_endpoints(service_config: &mut ServiceConfig) {
    service_config
        .route("/", get().to(get_index_html))
        .route("/favicon.png", get().to(get_index_favicon));
}

async fn get_index_html() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(INDEX_HTML.get_or_init(||
            process_template(
                include_str!("assets/index.template.html"),
                &[
                    ("APP_NAME", config::APP_NAME),
                    ("APP_VERSION", config::APP_VERSION),
                    ("APP_HOMEPAGE", config::APP_HOMEPAGE)
                ]
            )
        ).as_str())
}

async fn get_index_favicon() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::png())
        .body(Bytes::from_static(include_bytes!("assets/index-favicon.png")))
}
