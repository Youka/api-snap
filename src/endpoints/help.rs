use std::sync::OnceLock;
use actix_web::web::{
    get,
    redirect
};
use pulldown_cmark::{
    html as cmark_html,
    Options as cmark_options,
    Parser as cmark_parser
};
use super::models::http::{
    ContentType,
    HttpResponse,
    Responder,
    ServiceConfig
};
use crate::utils::string::process_template;

static KUBERNETES_SETUP_HTML: OnceLock<String> = OnceLock::new();

pub fn configure_help_endpoints(service_config: &mut ServiceConfig) {
    service_config
        .service(redirect("/help", "/help/kubernetes-setup"))
        .route("/help/kubernetes-setup", get().to(get_kubernetes_setup_html));
}

async fn get_kubernetes_setup_html() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(KUBERNETES_SETUP_HTML.get_or_init(|| {
            let mut output = String::new();
            cmark_html::push_html(
                &mut output,
                cmark_parser::new_ext(
                    include_str!("../../docs/kubernetes_setup.md"),
                    cmark_options::all()
                )
            );
            process_template(
                include_str!("assets/help.template.html"),
                &[
                    ("TITLE", "Kubernetes setup"),
                    ("BODY", &output)
                ]
            )
        }).as_str())
}
