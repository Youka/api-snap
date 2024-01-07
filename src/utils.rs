use std::{
    collections::HashMap,
    env::var as env_var
};
use actix_web::{
    web::Bytes,
    HttpRequest
};
use awc::Client;
use log::warn;
use crate::constants;

pub fn labels_to_map(labels: &[(&str, &str)]) -> HashMap<String,String> {
    labels.iter()
        .map(|label| (label.0.to_owned(), label.1.to_owned()))
        .collect()
}

pub fn env_var_as_string(name: &str) -> Option<String> {
    env_var(constants::env_var_prefix!() + name).ok()
}

pub fn env_var_as_u16(name: &str) -> Option<u16> {
    env_var(constants::env_var_prefix!() + name).ok().and_then(|var| var.parse().ok())
}

pub fn process_template(template: &str, vars: &[(&str,&str)]) -> String {
    vars.iter().fold(
        template.to_owned(),
        |output, (key, value)| output.replace(&format!("{{{{{}}}}}", key), value)
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

pub fn extract_http_url(request: HttpRequest) -> String {
    let connection_info = request.connection_info();
    format!(
        "{}://{}{}",
        connection_info.scheme(),
        connection_info.host(),
        request.path()
    )
}
