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
use serde::{
    Deserialize,
    Serialize
};
use crate::{
    constants,
    utils
};

pub fn configure_asyncapi_endpoints(service_config: &mut ServiceConfig) {
    service_config
        .service(redirect("/asyncapi", "/asyncapi/index.html"))
        .route("/asyncapi/index.html", get().to(get_asyncapi_index))
        .route("/asyncapi/urls", get().to(get_asyncapi_urls))
        .route("/asyncapi/document", get().to(get_asyncapi_document))
        .service(Files::new("/asyncapi", concat!(constants::third_party_dir!(), "/asyncapi-react/")));
}

async fn get_asyncapi_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("assets/asyncapi-index.html"))
}

async fn get_asyncapi_urls(request: HttpRequest) -> impl Responder {
    let url = utils::extract_http_url(request);
    let base_url = url.strip_suffix("/urls").expect("Http request matches route registration");
    Json([
        AsyncApiUrl {
            name: "Streetlights Kafka API".to_owned(),
            url: format!("{}/document?namespace=default&service=streetlights_kafka", base_url)
        },
        AsyncApiUrl {
            name: "Streetlights MQTT API".to_owned(),
            url: format!("{}/document?namespace=default&service=streetlights_mqtt", base_url)
        },
    ])
}

async fn get_asyncapi_document(query: Query<DocumentQuery>) -> impl Responder {
    match query.into_inner() {
        DocumentQuery { ref namespace, ref service } if namespace == "default" && service == "streetlights_kafka" =>
            utils::http_get("https://raw.githubusercontent.com/asyncapi/spec/v3.0.0/examples/streetlights-kafka-asyncapi.yml").await,
        DocumentQuery { ref namespace, ref service } if namespace == "default" && service == "streetlights_mqtt" =>
            utils::http_get("https://raw.githubusercontent.com/asyncapi/spec/v3.0.0/examples/streetlights-mqtt-asyncapi.yml").await,
        _ => None
    }
}

#[derive(Serialize)]
struct AsyncApiUrl {
    name: String,
    url: String
}

#[derive(Deserialize)]
struct DocumentQuery {
    namespace: String,
    service: String
}
