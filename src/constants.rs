pub const DEFAULT_ADDRESS: &str = "127.0.0.1";
pub const DEFAULT_PORT: u16 = 8080;

macro_rules! env_var_prefix { () => { "API_SNAP_"} }
pub(crate) use env_var_prefix;

macro_rules! third_party_dir { () => { "third-party" } }
pub(crate) use third_party_dir;
