use actix_web::{
    http::StatusCode,
    web::{
        get,
        redirect,
        Data,
        Json,
        ServiceConfig
    },
    Responder,
};
use serde::Serialize;
use crate::k8s_client::K8sClient;

pub fn configure_health_endpoints(service_config: &mut ServiceConfig) {
    service_config
        .service(redirect("/health", "/health/live"))
        .route("/health/live", get().to(get_health_live))
        .route("/health/ready", get().to(get_health_ready));
}

async fn get_health_live() -> impl Responder {
    ""
}

async fn get_health_ready(k8s_client: Data<K8sClient>) -> impl Responder {
    let mut is_ready = true;

    let k8s_status = match k8s_client.get_server_version().await {
        Ok(info) => format!("{:?}", info),
        Err(err) => {
            is_ready = false;
            err.to_string()
        }
    };

    (
        Json(ReadyStatus { k8s: k8s_status }),
        if is_ready { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE }
    )
}

#[derive(Serialize)]
struct ReadyStatus {
    k8s: String
}
