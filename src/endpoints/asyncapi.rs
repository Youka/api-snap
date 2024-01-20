use actix_files::Files;
use actix_web::{
    http::{
        header::ContentType,
        StatusCode
    },
    web::{
        get,
        redirect,
        Data,
        Json,
        Query,
        ServiceConfig
    },
    HttpRequest,
    HttpResponse,
    Responder
};
use log::error;
use serde::{
    Deserialize,
    Serialize
};
use crate::{
    clients::{
        k8s_client::K8sClient,
        documents_proxies::{
            get_api_services,
            ApiType
        }
    },
    config,
    utils::http::{
        extract_http_url,
        http_get
    }
};

pub fn configure_asyncapi_endpoints(service_config: &mut ServiceConfig) {
    service_config
        .service(redirect("/asyncapi", "/asyncapi/index.html"))
        .route("/asyncapi/index.html", get().to(get_asyncapi_index))
        .route("/asyncapi/urls", get().to(get_asyncapi_urls))
        .route("/asyncapi/document", get().to(get_asyncapi_document))
        .service(Files::new("/asyncapi", concat!(config::third_party_dir!(), "/asyncapi-react/")));
}

async fn get_asyncapi_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("assets/asyncapi-index.html"))
}

async fn get_asyncapi_urls(request: HttpRequest, k8s_client: Data<K8sClient>) -> impl Responder {
    let url = extract_http_url(request);
    let base_url = url.strip_suffix("/urls").expect("Http request matches route registration");

    match get_api_services(&k8s_client, ApiType::ASYNCAPI).await {
        Ok(services) => (
            Json(
                services.into_iter()
                    .map(|(namespace, name)| AsyncApiUrl {
                        name: format!("{}/{}", namespace, name),
                        url: format!("{}/document?namespace={}&service={}", base_url, namespace, name)
                    })
                    .collect()
            ),
            StatusCode::OK
        ),
        Err(err) => {
            error!("Finding AsyncAPI services failed: {}", err);
            (
                Json(vec![]),
                StatusCode::INTERNAL_SERVER_ERROR
            )
        }
    }
}

async fn get_asyncapi_document(query: Query<DocumentQuery>) -> impl Responder {

    // TODO: implement by k8s

    match query.into_inner() {
        DocumentQuery { ref namespace, ref service } if namespace == "default" && service == "streetlights_kafka" =>
            http_get("https://raw.githubusercontent.com/asyncapi/spec/v3.0.0/examples/streetlights-kafka-asyncapi.yml").await,
        DocumentQuery { ref namespace, ref service } if namespace == "default" && service == "streetlights_mqtt" =>
            http_get("https://raw.githubusercontent.com/asyncapi/spec/v3.0.0/examples/streetlights-mqtt-asyncapi.yml").await,
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
