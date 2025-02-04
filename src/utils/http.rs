use std::time::Duration;
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
use crate::config;

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
    let client = Client::builder()
        .timeout(Duration::from_secs(config::get_client_timeout().into()))
        .finish();
    match client.get(url).send().await {
        Ok(mut response) => response.body().await
            .map_err(|err| anyhow!("Http body invalid: {} => {}", url, err)),
        Err(err) => bail!("Http get request failed: {} => {}", url, err)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::spawn;
    use actix_web::{
        rt::System,
        test::TestRequest,
        web::get,
        App,
        HttpServer
    };

    #[test]
    fn extract_http_url_short_example() {
        assert_eq!(
            "http://example.com/",
            extract_http_url(TestRequest::default()
                .uri("http://example.com")
                .to_http_request()
            )
        );
    }

    #[test]
    fn extract_http_url_long_example() {
        assert_eq!(
            "http://example.com/test",
            extract_http_url(TestRequest::default()
                .uri("http://example.com/test?foo=bar")
                .to_http_request()
            )
        );
    }

    #[test]
    fn http_get_example() {
        const TEST_BIND: (&str, u16) = ("127.0.0.1", 9100);
        const EXAMPLE_BODY: &str = "example";

        let server = HttpServer::new(||
            App::new().route("/", get().to(|| async { EXAMPLE_BODY }))
        ).bind(TEST_BIND).unwrap().run();
        let server_handle = server.handle();

        let server_thread = spawn(|| {
            System::new().block_on(server).unwrap();
        });

        System::new().block_on(async {
            assert_eq!(
                EXAMPLE_BODY.as_bytes().to_vec(),
                http_get(&format!("http://{}:{}", TEST_BIND.0, TEST_BIND.1)).await.unwrap().to_vec()
            );

            assert!(http_get(&format!("http://{}:{}", TEST_BIND.0, TEST_BIND.1 + 1)).await.is_err(), "Http get request must fail!");

            server_handle.stop(true).await;
            server_thread.join().unwrap();
        });
    }
}
