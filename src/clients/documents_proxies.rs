use actix_web::web::Bytes;
use anyhow::Result as AnyResult;
use cached::{
    proc_macro::cached,
    TimedCache
};
use log::debug;
use super::k8s_client::K8sClient;
use crate::{
    config,
    utils::http::http_get
};

#[cached(
    result = true,
    sync_writes = true,
    key = "&'static str",
    convert = r#"{ "" }"#,
    type = "TimedCache<&'static str, String>",
    create = "{ TimedCache::with_lifespan(config::get_cache_lifespan().into()) }"
)]
pub async fn get_k8s_status(k8s_client: &K8sClient) -> AnyResult<String> {
    debug!("Cache hit: get_k8s_status");
    Ok(format!("{:?}", k8s_client.get_server_version().await?))
}

#[cached(
    result = true,
    sync_writes = true,
    key = "ApiType",
    convert = r#"{ api_type }"#,
    type = "TimedCache<ApiType, Vec<(String, String)>>",
    create = "{ TimedCache::with_lifespan(config::get_cache_lifespan().into()) }"
)]
pub async fn get_api_services(k8s_client: &K8sClient, api_type: ApiType) -> AnyResult<Vec<(String, String)>> {
    debug!("Cache hit: find_api_services, {:?}", api_type);
    k8s_client.get_services_with_any_annotation(&match api_type {
        ApiType::ASYNCAPI => [
            config::ASYNCAPI_PORT_ANNOTATION,
            config::ASYNCAPI_PATH_ANNOTATION
        ],
        ApiType::OPENAPI => [
            config::OPENAPI_PORT_ANNOTATION,
            config::OPENAPI_PATH_ANNOTATION
        ]
    }).await
}

#[cached(
    result = true,
    sync_writes = true,
    key = "(ApiType, String, String)",
    convert = r#"{ (api_type, namespace.to_owned(), name.to_owned()) }"#,
    type = "TimedCache<(ApiType, String, String), Bytes>",
    create = "{ TimedCache::with_lifespan(config::get_cache_lifespan().into()) }"
)]
pub async fn get_service_api_content(k8s_client: &K8sClient, api_type: ApiType, namespace: &str, name: &str) -> AnyResult<Bytes> {
    http_get(
        &match api_type {
            ApiType::ASYNCAPI => k8s_client.get_service_url_by_annotated_port_and_path(namespace, name, config::ASYNCAPI_PORT_ANNOTATION, 80, config::ASYNCAPI_PATH_ANNOTATION, "/openapi"),
            ApiType::OPENAPI => k8s_client.get_service_url_by_annotated_port_and_path(namespace, name, config::OPENAPI_PORT_ANNOTATION, 80, config::OPENAPI_PATH_ANNOTATION, "/asyncapi")
        }.await?
    ).await
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum ApiType {
    ASYNCAPI,
    OPENAPI
}
