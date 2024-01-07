use k8s_openapi::apimachinery::pkg::version::Info;
use kube::{
    Client,
    Result as KubeResult
};

#[derive(Clone)]
pub struct K8sClient {
    client: Client
}

impl K8sClient {
    pub async fn new() -> KubeResult<Self> {
        Ok(Self {
            client: Client::try_default().await?
        })
    }

    pub async fn get_server_version(&self) -> KubeResult<Info> {
        self.client.apiserver_version().await
    }
}
