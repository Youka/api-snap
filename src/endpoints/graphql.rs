use actix_web::web::{
    get,
    redirect
};
use log::{
    error,
    warn
};
use super::models::{
    api::{
        ApiUrl,
        DocumentQuery
    },
    http::{
        Bytes,
        ContentType,
        Data,
        Files,
        HttpRequest,
        HttpResponse,
        Json,
        Query,
        Responder,
        ServiceConfig,
        StatusCode
    }
};
use crate::{
    clients::{
        documents_proxies::{
            get_api_services,
            get_service_api_content,
            ApiType
        },
        k8s_client::K8sClient
    },
    config,
    utils::http::extract_http_url
};

pub fn configure_graphql_endpoints(service_config: &mut ServiceConfig) {
    service_config
        .service(redirect("/graphql", "/graphql/index.html"))
        .route("/graphql/index.html", get().to(get_graphql_index))
        .route("/graphql/urls", get().to(get_graphql_urls))
        .route("/graphql/document", get().to(get_graphql_document))
        .service(Files::new("/graphql", concat!(config::third_party_dir!(), "/prism-graphql/")));
}

async fn get_graphql_index() -> impl Responder {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("assets/graphql-index.html"))
}

async fn get_graphql_urls(request: HttpRequest, k8s_client: Data<K8sClient>) -> impl Responder {
    let url = extract_http_url(request);
    let base_url = url.strip_suffix("/urls").expect("Http request matches route registration");

    match get_api_services(&k8s_client, ApiType::Graphql).await {
        Ok(services) => (
            Json(
                services.into_iter()
                    .map(|(namespace, name)| ApiUrl {
                        name: format!("{}/{}", namespace, name),
                        url: format!("{}/document?namespace={}&service={}", base_url, namespace, name)
                    })
                    .collect()
            ),
            StatusCode::OK
        ),
        Err(err) => {
            error!("Getting GraphQL services failed: {}", err);
            (
                Json(vec![]),
                StatusCode::BAD_GATEWAY
            )
        }
    }
}

async fn get_graphql_document(query: Query<DocumentQuery>, k8s_client: Data<K8sClient>) -> impl Responder {
    match get_service_api_content(&k8s_client, ApiType::Graphql, &query.namespace, &query.service).await {
        Ok(bytes) => (
            bytes,
            StatusCode::OK
        ),
        Err(err) => {
            warn!("Getting GraphQL document failed: {}", err);
            (
                Bytes::new(),
                StatusCode::BAD_GATEWAY
            )
        }
    }
}
