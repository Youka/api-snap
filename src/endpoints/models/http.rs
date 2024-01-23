pub use actix_files::Files;
pub use actix_web::{
    http::{
        header::ContentType,
        StatusCode
    },
    web::{
        Bytes,
        Data,
        Json,
        Query,
        ServiceConfig
    },
    HttpRequest,
    HttpResponse,
    Responder
};
pub use mime::APPLICATION_JAVASCRIPT;
