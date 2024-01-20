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
