use std::env::var as env_var;
use super::string::LoggedParse;

pub fn env_var_as_string(name: &str) -> Option<String> {
    env_var(name).ok()
}

pub fn env_var_as_u16(name: &str) -> Option<u16> {
    env_var_as_string(name).and_then(|var| var.logged_parse(&format!("Environment variable '{}={}'", name, var)))
}
