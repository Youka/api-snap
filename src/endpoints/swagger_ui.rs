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
use mime::APPLICATION_JAVASCRIPT;
use serde::{
    Deserialize,
    Serialize
};
use crate::{
    clients::{
        documents_proxies::{
            get_api_services,
            get_service_api_content,
            ApiType
        },
        k8s_client::K8sClient
    },
    config,
    utils::http::extract_http_url
};

pub fn configure_swagger_ui_endpoints(service_config: &mut ServiceConfig) {
    service_config
        .service(redirect("/swagger-ui", "/swagger-ui/index.html"))
        .route("/swagger-ui/swagger-initializer.js", get().to(get_swagger_initializer))
        .route("/swagger-ui/urls", get().to(get_swagger_ui_urls))
        .route("/swagger-ui/document", get().to(get_openapi_document))
        .service(Files::new("/swagger-ui", concat!(config::third_party_dir!(), "/swagger-ui/")));
}

async fn get_swagger_initializer() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType(APPLICATION_JAVASCRIPT))
        .body(include_str!("assets/swagger-initializer.js"))
}

async fn get_swagger_ui_urls(request: HttpRequest, k8s_client: Data<K8sClient>) -> impl Responder {
    let url = extract_http_url(request);
    let base_url = url.strip_suffix("/urls").expect("Http request matches route registration");

    match get_api_services(&k8s_client, ApiType::OPENAPI).await {
        Ok(services) => (
            Json(
                services.into_iter()
                    .map(|(namespace, name)| SwaggerUiUrl {
                        name: format!("{}/{}", namespace, name),
                        url: format!("{}/document?namespace={}&service={}", base_url, namespace, name)
                    })
                    .collect()
            ),
            StatusCode::OK
        ),
        Err(err) => {
            error!("Getting OpenAPI services failed: {}", err);
            (
                Json(vec![]),
                StatusCode::BAD_GATEWAY
            )
        }
    }
}

async fn get_openapi_document(query: Query<DocumentQuery>, k8s_client: Data<K8sClient>) -> impl Responder {
    match get_service_api_content(&k8s_client, ApiType::OPENAPI, &query.namespace, &query.service).await {
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
struct SwaggerUiUrl {
    name: String,
    url: String
}

#[derive(Deserialize)]
struct DocumentQuery {
    namespace: String,
    service: String
}
