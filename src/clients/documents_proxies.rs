use anyhow::Result as AnyResult;
use cached::{
    proc_macro::cached,
    TimedCache
};
use super::k8s_client::K8sClient;
use crate::config;

#[cached(
    result = true,
    sync_writes = true,
    key = "&'static str",
    convert = r#"{ "k8s" }"#,
    type = "TimedCache<&'static str, String>",
    create = "{ TimedCache::with_lifespan(config::get_cache_lifespan().into()) }"
)]
pub async fn get_k8s_status(k8s_client: &K8sClient) -> AnyResult<String> {
    Ok(format!("{:?}", k8s_client.get_server_version().await?))
}
