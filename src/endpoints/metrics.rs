use actix_web_prom::{
    PrometheusMetrics,
    PrometheusMetricsBuilder
};
use prometheus::default_registry;
use crate::{
    constants,
    utils::collections::labels_to_map
};

pub fn build_prometheus_metrics_middleware() -> Result<PrometheusMetrics, Box<dyn std::error::Error + Send + Sync>> {
    PrometheusMetricsBuilder::new(&constants::app_namespace!())
        .const_labels(labels_to_map(&[
            ("app_name", constants::APP_NAME),
            ("app_version", constants::APP_VERSION)
        ]))
        .endpoint("/metrics")
        .registry(default_registry().clone())
        .build()
}
