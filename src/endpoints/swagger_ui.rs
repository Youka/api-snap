use actix_web::web::{
    get,
    redirect
};
use log::{
    error,
    warn
};
use super::models::{
    api::{
        ApiUrl,
        DocumentQuery
    },
    http::{
        APPLICATION_JAVASCRIPT,
        Bytes,
        ContentType,
        Data,
        Files,
        HttpRequest,
        HttpResponse,
        Json,
        Query,
        Responder,
        ServiceConfig,
        StatusCode
    }
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

    match get_api_services(&k8s_client, ApiType::Openapi).await {
        Ok(services) => (
            Json(
                services.into_iter()
                    .map(|(namespace, name)| ApiUrl {
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
    match get_service_api_content(&k8s_client, ApiType::Openapi, &query.namespace, &query.service).await {
        Ok(bytes) => (
            bytes,
            StatusCode::OK
        ),
        Err(err) => {
            warn!("Getting OpenAPI document failed: {}", err);
            (
                Bytes::new(),
                StatusCode::BAD_GATEWAY
            )
        }
    }
}
