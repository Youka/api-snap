use std::env::var as env_var;
use anyhow::Result as AnyResult;
use k8s_openapi::apimachinery::pkg::version::Info;
use kube::Client;

#[derive(Clone)]
pub struct K8sClient {
    client: Client
}

impl K8sClient {
    pub async fn new() -> AnyResult<Self> {
        Ok(Self {
            client: Client::try_default().await?,
        })
    }

    pub async fn get_server_version(&self) -> AnyResult<Info> {
        Ok(self.client.apiserver_version().await?)
    }
}

fn is_incluster() -> bool {
    env_var("KUBERNETES_SERVICE_HOST").is_ok()
}
