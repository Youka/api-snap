use crate::utils::env;

// Project
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");

// Static
macro_rules! third_party_dir { () => { "third-party" } }
pub(crate) use third_party_dir;

macro_rules! port_annotation_postfix { () => { "port" } }
macro_rules! path_annotation_postfix { () => { "path" } }
macro_rules! openapi_annotation_prefix { () => { "openapi/" } }
macro_rules! asyncapi_annotation_prefix { () => { "asyncapi/" } }
macro_rules! graphql_annotation_prefix { () => { "graphql/" } }

pub const OPENAPI_PORT_ANNOTATION: &str = concat!(openapi_annotation_prefix!(), port_annotation_postfix!());
pub const OPENAPI_PATH_ANNOTATION: &str = concat!(openapi_annotation_prefix!(), path_annotation_postfix!());
pub const ASYNCAPI_PORT_ANNOTATION: &str = concat!(asyncapi_annotation_prefix!(), port_annotation_postfix!());
pub const ASYNCAPI_PATH_ANNOTATION: &str = concat!(asyncapi_annotation_prefix!(), path_annotation_postfix!());
pub const GRAPHQL_PORT_ANNOTATION: &str = concat!(graphql_annotation_prefix!(), port_annotation_postfix!());
pub const GRAPHQL_PATH_ANNOTATION: &str = concat!(graphql_annotation_prefix!(), path_annotation_postfix!());
pub const DEFAULT_API_PORT: u16 = 80;
pub const DEFAULT_OPENAPI_PATH: &str = "/openapi";
pub const DEFAULT_ASYNCAPI_PATH: &str = "/asyncapi";
pub const DEFAULT_GRAPHQL_PATH: &str = "/graphql";

// Dynamic
macro_rules! env_var_prefix { () => { env!("CARGO_PKG_NAME").to_uppercase().replace("-", "_") + "_" } }

pub fn get_address() -> String {
    env::env_var_as_string(&(env_var_prefix!() + "ADDRESS")).unwrap_or("127.0.0.1".to_owned())
}
pub fn get_port() -> u16 {
    env::env_var_as_u16(&(env_var_prefix!() + "PORT")).unwrap_or(8080)
}
pub fn get_client_timeout() -> u16 {
    env::env_var_as_u16(&(env_var_prefix!() + "CLIENT_TIMEOUT")).unwrap_or(30)
}
pub fn get_cache_lifespan() -> u16 {
    env::env_var_as_u16(&(env_var_prefix!() + "CACHE_LIFESPAN")).unwrap_or(10)
}
