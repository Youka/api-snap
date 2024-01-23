use serde::{
    Deserialize,
    Serialize
};

#[derive(Serialize)]
pub struct ApiUrl {
    pub name: String,
    pub url: String
}

#[derive(Deserialize)]
pub struct DocumentQuery {
    pub namespace: String,
    pub service: String
}
