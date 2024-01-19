use anyhow::{
    anyhow,
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
    utils::{
        env::env_var_as_u16,
        string::LoggedParse
    }
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
            timeout_seconds: env_var_as_u16("CLIENT_TIMEOUT").unwrap_or(constants::DEFAULT_CLIENT_TIMEOUT).into()
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

    pub async fn get_service_url_by_annotated_port_and_path(
        &self,
        namespace: &str, name: &str,
        port_annotation: &str, default_port: u16,
        path_annotation: &str, default_path: &str
    ) -> AnyResult<String> {
        let service = Api::<Service>::namespaced(self.client.clone(), namespace)
            .get(name).await?;

        let service_annotations = service.annotations();
        let service_spec = service.spec.as_ref()
            .ok_or(anyhow!("Service '{}/{}' missing specification", namespace, name))?;

        let mut port = service_annotations.get(port_annotation)
            .and_then(|var| var.logged_parse(&format!("Service '{}/{}' port annotation '{}={}'", namespace, name, port_annotation, var)))
            .unwrap_or(default_port);
        let path = service_annotations.get(path_annotation)
            .map(|path| path.as_str())
            .unwrap_or(default_path);
        let host = if self.incluster {
            service_spec.cluster_ip.as_ref()
                .ok_or(anyhow!("Service '{}/{}' missing cluster ip", namespace, name))?
        } else {
            if service_spec.type_ != Some("NodePort".to_owned()) {
                bail!("Service '{}/{}' requires to be of type NodePort for non-incluster communication", namespace, name);
            }

            // TODO: update port by NodePort of fitting internal port

            service_spec.ports.as_ref()
                .ok_or(anyhow!("Service '{}/{}' requires to have ports for non-incluster communication", namespace, name));

            port = 0;//service_spec.ports;
            "localhost"
        };
        Ok(format!("http://{}:{}{}", host, port, path))
    }
}

pub struct ServiceId {
    pub namespace: String,
    pub name: String
}
