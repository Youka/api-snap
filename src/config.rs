use crate::utils::env;

// Project
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

// Static
pub const OPENAPI_PORT_ANNOTATION: &str = "openapi/port";
pub const OPENAPI_PATH_ANNOTATION: &str = "openapi/path";
pub const DEFAULT_OPENAPI_PATH: &str = "/openapi";
pub const ASYNCAPI_PORT_ANNOTATION: &str = "asyncapi/port";
pub const ASYNCAPI_PATH_ANNOTATION: &str = "asyncapi/path";
pub const DEFAULT_ASYNCAPI_PATH: &str = "/asyncapi";
pub const DEFAULT_API_PORT: u16 = 80;

macro_rules! env_var_prefix { () => { env!("CARGO_PKG_NAME").to_uppercase().replace("-", "_") + "_" } }
macro_rules! app_namespace { () => { env!("CARGO_PKG_NAME").to_lowercase().replace("-", "") } }
macro_rules! third_party_dir { () => { "third-party" } }

pub(crate) use {
    app_namespace,
    env_var_prefix,
    third_party_dir
};

// Dynamic
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
