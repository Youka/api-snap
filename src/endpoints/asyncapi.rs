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
        k8s_client::{
            K8sClient,
            ServiceId
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

    match get_api_services(&k8s_client, ApiType::Asyncapi).await {
        Ok(services) => (
            Json(
                services.into_iter()
                    .map(|ServiceId { namespace, name }| ApiUrl {
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
    match get_service_api_content(&k8s_client, ApiType::Asyncapi, &query.namespace, &query.service).await {
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
