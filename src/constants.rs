// Project
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

// Web
pub const DEFAULT_ADDRESS: &str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 8080;

// Environment
macro_rules! env_var_prefix { () => { env!("CARGO_PKG_NAME").to_uppercase().replace("-", "_") + "_" } }
macro_rules! app_namespace { () => { env!("CARGO_PKG_NAME").to_lowercase().replace("-", "") } }
macro_rules! third_party_dir { () => { "third-party" } }

pub(crate) use {
    app_namespace,
    env_var_prefix,
    third_party_dir
};
