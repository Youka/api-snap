use anyhow::Result as AnyResult;
use super::k8s_client::K8sClient;

#[derive(Clone)]
pub struct DocumentsClient {
    k8s_client: K8sClient
}

impl DocumentsClient {
    pub async fn new() -> AnyResult<Self> {
        Ok(Self {
            k8s_client: K8sClient::new().await?
        })
    }

    pub async fn get_k8s_status(&self) -> AnyResult<String> {
        Ok(format!("{:?}", self.k8s_client.get_server_version().await?))
    }
}
