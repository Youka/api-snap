use std::collections::HashMap;

pub fn labels_to_map(labels: &[(&str, &str)]) -> HashMap<String,String> {
    labels.iter()
        .map(|label| (label.0.to_owned(), label.1.to_owned()))
        .collect()
}
