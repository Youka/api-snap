use actix_web::{
    Responder,
    web::{
        get,
        Json,
        ServiceConfig
    }, HttpResponse, http::header::ContentType
};
use actix_files::Files;
use mime::APPLICATION_JAVASCRIPT;
use serde::Serialize;

pub fn configure_swagger_ui_services(service_config: &mut ServiceConfig) {
    service_config
        .route("/swagger-ui/swagger-initializer.js", get().to(get_swagger_initializer))
        .route("/swagger-ui/urls", get().to(get_swagger_ui_urls))
        .service(Files::new("/swagger-ui/", "./third-party/swagger-ui/").redirect_to_slash_directory().index_file("index.html"));
}

async fn get_swagger_initializer() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType(APPLICATION_JAVASCRIPT))
        .body(include_str!("swagger-initializer.js"))
}

async fn get_swagger_ui_urls() -> impl Responder {
    Json([
        SwaggerUiUrl {
            name: "Petstore v2".to_owned(),
            url: "https://petstore.swagger.io/v2/swagger.json".to_owned()
        },
        SwaggerUiUrl {
            name: "Petstore v3".to_owned(),
            url: "https://petstore3.swagger.io/api/v3/openapi.json".to_owned()
        },
    ])
}

#[derive(Serialize)]
struct SwaggerUiUrl {
    name: String,
    url: String
}
