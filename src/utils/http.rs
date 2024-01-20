use actix_web::{
    web::Bytes,
    HttpRequest
};
use awc::Client;
use log::warn;

pub fn extract_http_url(request: HttpRequest) -> String {
    let connection_info = request.connection_info();
    format!(
        "{}://{}{}",
        connection_info.scheme(),
        connection_info.host(),
        request.path()
    )
}

pub async fn http_get(url: &str) -> Option<Bytes> {
    match Client::new().get(url).send().await {
        Ok(mut response) => match response.body().await {
            Ok(body) => Some(body),
            Err(err) => {
                warn!("Http body invalid: {} => {}", url, err);
                None
            }
        },
        Err(err) => {
            warn!("Http get request failed: {} => {}", url, err);
            None
        }
    }
}
