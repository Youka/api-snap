use std::{
    env::set_var as set_env_var,
    path::PathBuf,
    sync::mpsc::channel,
    thread::spawn,
    time::Duration
};
use actix_web::{
    http::StatusCode,
    rt::System
};
use awc::Client;
use api_snap::main;

const TEST_PORT: u16 = 9101;

#[test]
fn main_no_k8s() {
    set_env_var("KUBECONFIG", PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("invalid_kubeconfig.yml").to_str().unwrap());
    set_env_var("API_SNAP_PORT", TEST_PORT.to_string());

    let (server_handle_sender, server_handle_receiver) = channel();
    let main_thread = spawn(|| main(Some(server_handle_sender)).unwrap());
    let server_handle = server_handle_receiver.recv_timeout(Duration::from_secs(5)).unwrap();

    System::new().block_on(async {
        let http_client = Client::default();
        test_index(&http_client).await;
        test_health(&http_client).await;
        test_help(&http_client).await;
        test_metrics(&http_client).await;
        test_asyncapi(&http_client).await;
        test_graphql(&http_client).await;
        test_swagger_ui(&http_client).await;

        server_handle.stop(true).await;
        main_thread.join().unwrap();
    });
}

async fn test_index(http_client: &Client) {
    http_get_check(&http_client, "/", StatusCode::OK).await;
    http_get_check(&http_client, "/favicon.png", StatusCode::OK).await;
}

async fn test_health(http_client: &Client) {
    http_get_check(&http_client, "/health", StatusCode::OK).await;
    http_get_check(&http_client, "/health/ready", StatusCode::SERVICE_UNAVAILABLE).await;  // No kubernetes server by test configuration
}

async fn test_help(http_client: &Client) {
    http_get_check(&http_client, "/help", StatusCode::OK).await;
}

async fn test_metrics(http_client: &Client) {
    http_get_check(&http_client, "/metrics", StatusCode::OK).await;
}

async fn test_asyncapi(http_client: &Client) {
    http_get_check(&http_client, "/asyncapi", StatusCode::OK).await;
    http_get_check(&http_client, "/asyncapi/urls", StatusCode::BAD_GATEWAY).await;
    http_get_check(&http_client, "/asyncapi/document?namespace=test&service=test", StatusCode::BAD_GATEWAY).await;
}

async fn test_graphql(http_client: &Client) {
    http_get_check(&http_client, "/graphql", StatusCode::OK).await;
    http_get_check(&http_client, "/graphql/urls", StatusCode::BAD_GATEWAY).await;
    http_get_check(&http_client, "/graphql/document?namespace=test&service=test", StatusCode::BAD_GATEWAY).await;
}

async fn test_swagger_ui(http_client: &Client) {
    http_get_check(&http_client, "/swagger-ui/swagger-initializer.js", StatusCode::OK).await;
    http_get_check(&http_client, "/swagger-ui/urls", StatusCode::BAD_GATEWAY).await;
    http_get_check(&http_client, "/swagger-ui/document?namespace=test&service=test", StatusCode::BAD_GATEWAY).await;
}

async fn http_get_check(http_client: &Client, path: &str, status_code: StatusCode) {
    assert_eq!(
        status_code,
        http_client.get(format!("http://localhost:{}{}", TEST_PORT, path)).send().await.unwrap().status()
    );
}
