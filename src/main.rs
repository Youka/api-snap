mod k8s;
mod constants;
mod ui;

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
    let addr = env_var(concat!(constants::env_var_prefix!(), "ADDRESS"))
        .unwrap_or(constants::DEFAULT_ADDRESS.to_owned());
    let port = env_var(concat!(constants::env_var_prefix!(), "PORT"))
        .ok().and_then(|var| var.parse().ok())
        .unwrap_or(constants::DEFAULT_PORT);

    // Start web server
    log::info!("Starting web server on '{}:{}'", addr, port);
    HttpServer::new(||
        App::new()
            .wrap(Compress::default())
            .wrap(Logger::default())
            .configure(ui::swagger_ui::configure_swagger_ui_services)
            .configure(ui::asyncapi::configure_asyncapi_services)
    )
    .bind((addr, port))?
    .run()
    .await
}
