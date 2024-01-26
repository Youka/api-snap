use std::env::var as env_var;
use super::string::LoggedParse;

pub fn env_var_as_string(name: &str) -> Option<String> {
    env_var(name).ok()
}

pub fn env_var_as_u16(name: &str) -> Option<u16> {
    env_var_as_string(name).and_then(|var| var.logged_parse(&format!("Environment variable '{}={}'", name, var)))
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env::{
        remove_var,
        set_var
    };

    #[test]
    fn env_var_as_string_none() {
        const TEST_KEY: &str = "env_var_as_string_none";
        remove_var(TEST_KEY);
        assert_eq!(None, env_var_as_string(TEST_KEY));
    }

    #[test]
    fn env_var_as_string_found() {
        const TEST_KEY: &str = "env_var_as_string_found";
        const TEST_VALUE_STR: &str = "ENV_TEST_VALUE";
        set_var(TEST_KEY, TEST_VALUE_STR);
        assert_eq!(Some(TEST_VALUE_STR.to_owned()), env_var_as_string(TEST_KEY));
    }

    #[test]
    fn env_var_as_u16_found() {
        const TEST_KEY: &str = "env_var_as_u16_found";
        const TEST_VALUE_NUMBER: u16 = 42;
        set_var(TEST_KEY, TEST_VALUE_NUMBER.to_string());
        assert_eq!(Some(TEST_VALUE_NUMBER), env_var_as_u16(TEST_KEY));
    }

    #[test]
    fn env_var_as_u16_parse_failed() {
        const TEST_KEY: &str = "env_var_as_u16_parse_failed";
        set_var(TEST_KEY, "");
        assert_eq!(None, env_var_as_u16(TEST_KEY));
    }
}
