use actix_web::{
    Responder,
    web::{
        get,
        Json,
        redirect,
        ServiceConfig
    },
    HttpResponse,
    http::header::ContentType
};
use actix_files::Files;
use serde::Serialize;

pub fn configure_asyncapi_services(service_config: &mut ServiceConfig) {
    service_config
        .service(redirect("/asyncapi", "/asyncapi/"))
        .service(redirect("/asyncapi/", "/asyncapi/index.html"))
        .route("/asyncapi/index.html", get().to(get_asyncapi_index))
        .route("/asyncapi/urls", get().to(get_asyncapi_urls))
        .service(Files::new("/asyncapi/", "./third-party/asyncapi-react/"));
}

async fn get_asyncapi_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("asyncapi-index.html"))
}

async fn get_asyncapi_urls() -> impl Responder {
    Json([
        AsyncApiUrl {
            name: "Streetlights Kafka API".to_owned(),
            url: "https://raw.githubusercontent.com/asyncapi/spec/v3.0.0/examples/streetlights-kafka-asyncapi.yml".to_owned()
        },
        AsyncApiUrl {
            name: "Streetlights MQTT API".to_owned(),
            url: "https://raw.githubusercontent.com/asyncapi/spec/v3.0.0/examples/streetlights-mqtt-asyncapi.yml".to_owned()
        },
    ])
}

#[derive(Serialize)]
struct AsyncApiUrl {
    name: String,
    url: String
}
