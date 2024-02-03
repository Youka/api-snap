mod clients;
mod config;
mod endpoints;
mod utils;

use std::{
    io::Result as IOResult,
    sync::mpsc::Sender
};
use actix_cors::Cors;
use actix_web::{
    dev::ServerHandle,
    main as actix_main,
    middleware::{
        Compress,
        Logger,
        NormalizePath
    },
    web::Data,
    App,
    HttpServer
};
#[cfg(feature = "dotenv")]
use dotenvy::dotenv;
use env_logger::{
    init_from_env as log_init,
    Env as LogEnvironment
};

#[actix_main]
pub async fn main(server_handle_sender: Option<Sender<ServerHandle>>) -> IOResult<()> {
    // Load environment variables from file
    #[cfg(feature = "dotenv")]
    if let Ok(path) = dotenv() {
        println!("Loaded .env file: {}", path.to_string_lossy());
    }

    // Initialize logging interface by environment variables
    log_init(LogEnvironment::default().default_filter_or("warn"));

    // Read configuration
    let address = config::get_address();
    let port = config::get_port();

    // Initialize shared web resources
    let metrics = endpoints::metrics::build_prometheus_metrics_middleware()
        .expect("Initialize prometheus metrics structure");
    let k8s_client = clients::k8s_client::K8sClient::new().await
        .expect("Initialize kubernetes client");

    // Start web server
    log::info!("Starting web server on '{}:{}'", address, port);
    let server = HttpServer::new(move ||
        App::new()
            .app_data(Data::new(k8s_client.clone()))
            .wrap(Compress::default())
            .wrap(Logger::default())
            .wrap(NormalizePath::trim())
            .wrap(Cors::default()
                .allow_any_origin()
                .send_wildcard()
                .allowed_methods(["GET"]))
            .wrap(metrics.clone())
            .configure(endpoints::asyncapi::configure_asyncapi_endpoints)
            .configure(endpoints::graphql::configure_graphql_endpoints)
            .configure(endpoints::swagger_ui::configure_swagger_ui_endpoints)
            .configure(endpoints::health::configure_health_endpoints)
            .configure(endpoints::index::configure_index_endpoints)
            .configure(endpoints::help::configure_help_endpoints)
    )
    .bind((address, port))?
    .run();

    if let Some(sender) = server_handle_sender {
        sender.send(server.handle()).expect("Server handle server still usable");
    }

    server.await
}
