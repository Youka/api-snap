use actix_files::Files;
use actix_web::{
    http::{
        header::ContentType,
        StatusCode
    },
    web::{
        get,
        redirect,
        Bytes,
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
            get_service_api_content,
            ApiType
        }
    },
    config,
    utils::http::extract_http_url
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
            error!("Getting AsyncAPI services failed: {}", err);
            (
                Json(vec![]),
                StatusCode::BAD_GATEWAY
            )
        }
    }
}

async fn get_asyncapi_document(query: Query<DocumentQuery>, k8s_client: Data<K8sClient>) -> impl Responder {
    match get_service_api_content(&k8s_client, ApiType::ASYNCAPI, &query.namespace, &query.service).await {
        Ok(bytes) => (
            bytes,
            StatusCode::OK
        ),
        Err(err) => {
            error!("Getting OpenAPI document failed: {}", err);
            (
                Bytes::new(),
                StatusCode::BAD_GATEWAY
            )
        }
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
