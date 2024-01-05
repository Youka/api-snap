mod k8s;
mod constants;
mod ui;
mod utils;

use std::{
    env::var as env_var,
    io::Result as IOResult
};
use actix_web::{
    main as actix_main,
    middleware::{
        Compress,
        Logger
    },
    App,
    HttpServer
};
use actix_web_prom::PrometheusMetricsBuilder;
use env_logger::{
    init_from_env as log_init,
    Env as LogEnvironment
};
use prometheus::default_registry;

#[actix_main]
async fn main() -> IOResult<()> {
    // Initialize logging interface by environment variables
    log_init(LogEnvironment::default().default_filter_or("info"));

    // Read configuration by environment variables
    let addr = env_var(constants::env_var_prefix!() + "ADDRESS")
        .unwrap_or(constants::DEFAULT_ADDRESS.to_owned());
    let port = env_var(constants::env_var_prefix!() + "PORT")
        .ok().and_then(|var| var.parse().ok())
        .unwrap_or(constants::DEFAULT_PORT);

    // Define prometheus metrics provider
    let metrics = PrometheusMetricsBuilder::new(&constants::app_namespace!())
        .const_labels(utils::labels_to_map(&[
            ("app_name", constants::APP_NAME),
            ("app_version", constants::APP_VERSION)
        ]))
        .endpoint("/metrics")
        .registry(default_registry().clone())
        .build().expect("Initialize prometheus metrics structure.");

    // Start web server
    log::info!("Starting web server on '{}:{}'", addr, port);
    HttpServer::new(move ||
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(metrics.clone())
            .configure(ui::swagger_ui::configure_swagger_ui_services)
            .configure(ui::asyncapi::configure_asyncapi_services)
    )
    .bind((addr, port))?
    .run()
    .await
}
