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
    type = "TimedCache<&'static str, String>",
    create = "{ TimedCache::with_lifespan(config::get_cache_lifespan().into()) }",
    convert = r#"{ "" }"#,
    result = true,
    sync_writes = true
)]
pub async fn get_k8s_status(k8s_client: &K8sClient) -> AnyResult<String> {
    debug!("Cache hit: get_k8s_status");
    Ok(format!("{:?}", k8s_client.get_server_version().await?))
}

#[cached(
    type = "TimedCache<ApiType, Vec<(String, String)>>",
    create = "{ TimedCache::with_lifespan(config::get_cache_lifespan().into()) }",
    convert = r#"{ api_type }"#,
    result = true,
    sync_writes = true
)]
pub async fn get_api_services(k8s_client: &K8sClient, api_type: ApiType) -> AnyResult<Vec<(String, String)>> {
    debug!("Cache hit: get_api_services, {:?}", api_type);
    k8s_client.get_services_with_any_annotation(&match api_type {
        ApiType::Asyncapi => [
            config::ASYNCAPI_PORT_ANNOTATION,
            config::ASYNCAPI_PATH_ANNOTATION
        ],
        ApiType::Graphql => [
            config::GRAPHQL_PORT_ANNOTATION,
            config::GRAPHQL_PATH_ANNOTATION
        ],
        ApiType::Openapi => [
            config::OPENAPI_PORT_ANNOTATION,
            config::OPENAPI_PATH_ANNOTATION
        ]
    }).await
}

#[cached(
    type = "TimedCache<(ApiType, String, String), Bytes>",
    create = "{ TimedCache::with_lifespan(config::get_cache_lifespan().into()) }",
    convert = r#"{ (api_type, namespace.to_owned(), name.to_owned()) }"#,
    result = true,
    sync_writes = true
)]
pub async fn get_service_api_content(k8s_client: &K8sClient, api_type: ApiType, namespace: &str, name: &str) -> AnyResult<Bytes> {
    debug!("Cache hit: get_service_api_content, {:?}, {}, {}", api_type, namespace, name);
    http_get(
        &match api_type {
            ApiType::Asyncapi => k8s_client.get_service_url_by_annotated_port_or_path(
                namespace, name,
                config::ASYNCAPI_PORT_ANNOTATION, config::DEFAULT_API_PORT,
                config::ASYNCAPI_PATH_ANNOTATION, config::DEFAULT_ASYNCAPI_PATH
            ),
            ApiType::Graphql => k8s_client.get_service_url_by_annotated_port_or_path(
                namespace, name,
                config::GRAPHQL_PORT_ANNOTATION, config::DEFAULT_API_PORT,
                config::GRAPHQL_PATH_ANNOTATION, config::DEFAULT_GRAPHQL_PATH
            ),
            ApiType::Openapi => k8s_client.get_service_url_by_annotated_port_or_path(
                namespace, name,
                config::OPENAPI_PORT_ANNOTATION, config::DEFAULT_API_PORT,
                config::OPENAPI_PATH_ANNOTATION, config::DEFAULT_OPENAPI_PATH
            )
        }.await?
    ).await
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum ApiType {
    Asyncapi,
    Graphql,
    Openapi
}
