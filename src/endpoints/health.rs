use actix_web::{
    web::{
        get,
        redirect,
        ServiceConfig
    },
    HttpResponse,
    Responder
};

pub fn configure_health_endpoints(service_config: &mut ServiceConfig) {
    service_config
        .service(redirect("/health", "/health/live"))
        .route("/health/live", get().to(get_health_live))
        .route("/health/ready", get().to(get_health_ready));
}

async fn get_health_live() -> impl Responder {
    HttpResponse::Ok()
}

async fn get_health_ready() -> impl Responder {
    HttpResponse::Ok()
}
