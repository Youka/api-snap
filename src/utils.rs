use std::{
    collections::HashMap,
    env::var as env_var
};
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
