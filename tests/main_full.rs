use std::{
    env::set_var as set_env_var,
    path::PathBuf,
    process::Command,
    sync::mpsc::channel,
    thread::{
        sleep,
        spawn
    },
    time::Duration
};
use actix_web::{
    http::StatusCode,
    rt::System
};
use awc::Client;
use api_snap::main;

const TEST_PORT: u16 = 9102;

#[test]
fn main_full() {
    apply_test_resources();
    test_main();
}

fn apply_test_resources() {
    kubectl_apply("k8s_test_namespaces.yml");
    sleep(Duration::from_secs(3));  // Give server some time to create namespaces first
    kubectl_apply("k8s_test_resources.yml");
}

fn kubectl_apply(test_file: &str) {
    let mut command = Command::new("kubectl");
    command.arg("apply")
        .arg("-f")
        .arg(test_file)
        .current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests"));
    if !command.status().unwrap().success() {
        panic!("Command failed: {:?}", command);
    }
}

fn test_main() {
    set_env_var("API_SNAP_PORT", TEST_PORT.to_string());

    let (server_handle_sender, server_handle_receiver) = channel();
    let main_thread = spawn(|| main(Some(server_handle_sender)).unwrap());
    let server_handle = server_handle_receiver.recv_timeout(Duration::from_secs(5)).unwrap();

    System::new().block_on(async {
        let http_client = Client::default();
        test_index(&http_client).await;
        test_health(&http_client).await;
        test_metrics(&http_client).await;
        test_asyncapi(&http_client).await;
        test_graphql(&http_client).await;
        test_swagger_ui(&http_client).await;

        server_handle.stop(true).await;
        main_thread.join().unwrap();
    });
}

async fn test_index(http_client: &Client) {
    http_get_check(&http_client, "/").await;
    http_get_check(&http_client, "/favicon.png").await;
}

async fn test_health(http_client: &Client) {
    http_get_check(&http_client, "/health").await;
    http_get_check(&http_client, "/health/ready").await;  // No kubernetes server by test configuration
}

async fn test_metrics(http_client: &Client) {
    http_get_check(&http_client, "/metrics").await;
}

async fn test_asyncapi(http_client: &Client) {
    http_get_check(&http_client, "/asyncapi").await;
    http_get_check(&http_client, "/asyncapi/urls").await;
    http_get_check(&http_client, "/asyncapi/document?namespace=test1&service=test-api-provider").await;
    http_get_check(&http_client, "/asyncapi/document?namespace=test2&service=test-api-provider").await;
}

async fn test_graphql(http_client: &Client) {
    http_get_check(&http_client, "/graphql").await;
    http_get_check(&http_client, "/graphql/urls").await;
    http_get_check(&http_client, "/graphql/document?namespace=test1&service=test-api-provider").await;
    http_get_check(&http_client, "/graphql/document?namespace=test2&service=test-api-provider").await;
}

async fn test_swagger_ui(http_client: &Client) {
    http_get_check(&http_client, "/swagger-ui/swagger-initializer.js").await;
    http_get_check(&http_client, "/swagger-ui/urls").await;
    http_get_check(&http_client, "/swagger-ui/document?namespace=test1&service=test-api-provider").await;
    http_get_check(&http_client, "/swagger-ui/document?namespace=test2&service=test-api-provider").await;
}

async fn http_get_check(http_client: &Client, path: &str) {
    assert_eq!(
        StatusCode::OK,
        http_client.get(format!("http://localhost:{}{}", TEST_PORT, path)).send().await.unwrap().status()
    );
}
