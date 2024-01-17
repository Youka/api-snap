use anyhow::{
    bail,
    Result as AnyResult
};
use k8s_openapi::{
    apimachinery::pkg::version::Info,
    api::core::v1::Service
};
use kube::{
    api::ListParams,
    config::KubeConfigOptions,
    Api,
    Client,
    Config,
    ResourceExt
};
use log::debug;
use crate::{
    constants,
    utils
};

#[derive(Clone)]
pub struct K8sClient {
    client: Client,
    incluster: bool,
    timeout_seconds: u32
}

impl K8sClient {
    pub async fn new() -> AnyResult<Self> {
        let (mut config, incluster) = match Config::incluster() {
            Ok(incluster_config) => (incluster_config, true),
            Err(err) => {
                debug!("Could not find an incluster config: {}", err);
                match Config::from_kubeconfig(&KubeConfigOptions::default()).await {
                    Ok(kube_config) => (kube_config, false),
                    Err(err) => {
                        debug!("Could also not find a local kubeconfig: {}", err);
                        bail!("Neither an incluster config nor a local kubeconfig was found!")
                    }
                }
            }
        };
        config.apply_debug_overrides();

        Ok(Self {
            client: Client::try_from(config)?,
            incluster,
            timeout_seconds: utils::env_var_as_u16("CLIENT_TIMEOUT").unwrap_or(constants::DEFAULT_CLIENT_TIMEOUT).into()
        })
    }

    pub async fn get_server_version(&self) -> AnyResult<Info> {
        Ok(self.client.apiserver_version().await?)
    }

    pub async fn get_services_with_any_annotation(&self, annotations: &[&str]) -> AnyResult<Vec<ServiceId>> {
        Ok(
            Api::<Service>::all(self.client.clone())
                .list(&ListParams::default().timeout(self.timeout_seconds)).await?
                .into_iter()
                .filter(|service| annotations.iter().any(|annotation| service.annotations().contains_key(*annotation)))
                .map(|service| ServiceId {
                    namespace: service.namespace().unwrap_or(self.client.default_namespace().to_owned()),
                    name: service.name_any()
                })
                .collect()
        )
    }
}

pub struct ServiceId {
    pub namespace: String,
    pub name: String
}
