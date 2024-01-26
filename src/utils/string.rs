use std::{
    fmt::Display,
    str::FromStr
};
use log::warn;

pub trait LoggedParse {
    fn logged_parse<R>(&self, context: &str) -> Option<R> where R: FromStr, <R as FromStr>::Err: Display;
}

impl LoggedParse for str {
    fn logged_parse<R>(&self, context: &str) -> Option<R> where R: FromStr, <R as FromStr>::Err: Display {
        match R::from_str(self) {
            Ok(result) => Some(result),
            Err(err) => {
                warn!("{}: {}", context, err);
                None
            }
        }

    }
}

pub fn process_template(template: &str, vars: &[(&str,&str)]) -> String {
    vars.iter().fold(
        template.to_owned(),
        |output, (key, value)| output.replace(&format!("{{{{{}}}}}", key), value)
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logged_parse_ok() {
        const TEST_VALUE: i32 = 1337;
        assert_eq!(Some(TEST_VALUE), TEST_VALUE.to_string().logged_parse("leet"));
    }

    #[test]
    fn logged_parse_fail() {
        assert_eq!(None, "".logged_parse::<u8>("empty"));
    }

    #[test]
    fn process_template_foo_bar() {
        assert_eq!(
            "Hello 'foo', or should i call you 'bar'?",
            process_template("Hello '{{placeholder1}}', or should i call you '{{placeholder2}}'?", &[("placeholder1", "foo"), ("placeholder2", "bar")])
        );
    }

    #[test]
    fn process_template_miss_placeholder() {
        const TEMPLATE: &str = "Hello {{test}}!";
        assert_eq!(
            TEMPLATE,
            process_template(TEMPLATE, &[])
        );
    }
}
