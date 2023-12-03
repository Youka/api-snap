use k8s_openapi::api::core::v1::Namespace;
use kube::{
    api::ListParams,
    Api,
    Client,
    ResourceExt
};

#[allow(dead_code)]
pub async fn create_client() {

    // TODO: implement correctly

    let client = Client::try_default().await.expect("Connection to the local kubernetes cluster.");
    let namespaces = Api::<Namespace>::all(client);
    for namespace in namespaces.list(&ListParams::default()).await.expect("Listing namespaces should be possible.") {
        println!("Namespace found: {}", namespace.name_any());
    }
}
