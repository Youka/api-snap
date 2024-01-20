use actix_web::{
    web::Bytes,
    HttpRequest
};
use anyhow::{
    anyhow,
    bail,
    Result as AnyResult
};
use awc::Client;

pub fn extract_http_url(request: HttpRequest) -> String {
    let connection_info = request.connection_info();
    format!(
        "{}://{}{}",
        connection_info.scheme(),
        connection_info.host(),
        request.path()
    )
}

pub async fn http_get(url: &str) -> AnyResult<Bytes> {
    match Client::new().get(url).send().await {
        Ok(mut response) => response.body().await
            .map_err(|err| anyhow!("Http body invalid: {} => {}", url, err)),
        Err(err) => bail!("Http get request failed: {} => {}", url, err)
    }
}
