use std::collections::HashMap;

pub fn labels_to_map(labels: &[(&str, &str)]) -> HashMap<String,String> {
    labels.iter()
        .map(|label| (label.0.to_owned(), label.1.to_owned()))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn labels_to_map_empty() {
        assert!(labels_to_map(&[]).is_empty())
    }

    #[test]
    fn labels_to_map_two_entries() {
        assert_eq!(
            labels_to_map(&[("foo", "bar"), ("hello", "world")]),
            [("hello".to_owned(), "world".to_owned()), ("foo".to_owned(), "bar".to_owned())].into_iter().collect()
        )
    }
}
