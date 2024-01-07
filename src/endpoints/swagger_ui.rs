use actix_files::Files;
use actix_web::{
    http::header::ContentType,
    web::{
        get,
        redirect,
        Json,
        Query,
        ServiceConfig
    },
    HttpRequest,
    HttpResponse,
    Responder
};
use mime::APPLICATION_JAVASCRIPT;
use serde::{
    Deserialize,
    Serialize
};
use crate::{
    constants,
    utils
};

pub fn configure_swagger_ui_endpoints(service_config: &mut ServiceConfig) {
    service_config
        .service(redirect("/swagger-ui", "/swagger-ui/index.html"))
        .route("/swagger-ui/swagger-initializer.js", get().to(get_swagger_initializer))
        .route("/swagger-ui/urls", get().to(get_swagger_ui_urls))
        .route("/swagger-ui/document", get().to(get_swagger_document))
        .service(Files::new("/swagger-ui", concat!(constants::third_party_dir!(), "/swagger-ui/")));
}

async fn get_swagger_initializer() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType(APPLICATION_JAVASCRIPT))
        .body(include_str!("assets/swagger-initializer.js"))
}

async fn get_swagger_ui_urls(request: HttpRequest) -> impl Responder {
    let url = utils::extract_http_url(request);
    let base_url = url.strip_suffix("/urls").expect("Http request matches route registration.");
    Json([
        SwaggerUiUrl {
            name: "Petstore v2".to_owned(),
            url: format!("{}/document?namespace=default&service=petstore", base_url)
        },
        SwaggerUiUrl {
            name: "Petstore v3".to_owned(),
            url: format!("{}/document?namespace=default&service=petstore3", base_url)
        },
    ])
}

async fn get_swagger_document(query: Query<DocumentQuery>) -> impl Responder {
    match query.into_inner() {
        DocumentQuery { ref namespace, ref service } if namespace == "default" && service == "petstore" =>
            utils::http_get("https://petstore.swagger.io/v2/swagger.json").await,
        DocumentQuery { ref namespace, ref service } if namespace == "default" && service == "petstore3" =>
            utils::http_get("https://petstore3.swagger.io/api/v3/openapi.json").await,
        _ => None
    }
}

#[derive(Serialize)]
struct SwaggerUiUrl {
    name: String,
    url: String
}

#[derive(Deserialize)]
struct DocumentQuery {
    namespace: String,
    service: String
}
