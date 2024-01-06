mod constants;
mod endpoints;
mod k8s;
mod utils;

use std::io::Result as IOResult;
use actix_web::{
    main as actix_main,
    middleware::{
        Compress,
        Logger,
        NormalizePath
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
    let address = utils::env_var_as_string("ADDRESS")
        .unwrap_or(constants::DEFAULT_ADDRESS.to_owned());
    let port = utils::env_var_as_u16("PORT")
        .unwrap_or(constants::DEFAULT_PORT);

    // Initialize middleware
    let metrics = endpoints::prometheus::build_prometheus_metrics_middleware()
        .expect("Initialize prometheus metrics structure.");

    // Start web server
    log::info!("Starting web server on '{}:{}'", address, port);
    HttpServer::new(move ||
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .wrap(metrics.clone())
            .configure(endpoints::asyncapi::configure_asyncapi_endpoints)
            .configure(endpoints::swagger_ui::configure_swagger_ui_endpoints)
            .configure(endpoints::health::configure_health_endpoints)
            .configure(endpoints::index::configure_index_endpoints)
    )
    .bind((address, port))?
    .run()
    .await
}
