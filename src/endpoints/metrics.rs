use actix_web_prom::{
    PrometheusMetrics,
    PrometheusMetricsBuilder
};
use prometheus::default_registry;
use crate::{
    config,
    utils::collections::labels_to_map
};

pub fn build_prometheus_metrics_middleware() -> Result<PrometheusMetrics, Box<dyn std::error::Error + Send + Sync>> {
    PrometheusMetricsBuilder::new(&config::APP_NAME.to_lowercase().replace('-', ""))
        .const_labels(labels_to_map(&[
            ("app_name", config::APP_NAME),
            ("app_version", config::APP_VERSION)
        ]))
        .endpoint("/metrics")
        .registry(default_registry().clone())
        .build()
}
