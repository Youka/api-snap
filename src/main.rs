mod constants;
mod endpoints;
mod k8s;
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
use env_logger::{
    init_from_env as log_init,
    Env as LogEnvironment
};

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

    // Initialize middleware
    let metrics = endpoints::prometheus::build_prometheus_metrics_middleware()
        .expect("Initialize prometheus metrics structure.");

    // Start web server
    log::info!("Starting web server on '{}:{}'", addr, port);
    HttpServer::new(move ||
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(metrics.clone())
            .configure(endpoints::asyncapi::configure_asyncapi_endpoints)
            .configure(endpoints::swagger_ui::configure_swagger_ui_endpoints)
            .configure(endpoints::health::configure_health_endpoints)
    )
    .bind((addr, port))?
    .run()
    .await
}
