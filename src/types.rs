use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub external_url: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Serialize, Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}
