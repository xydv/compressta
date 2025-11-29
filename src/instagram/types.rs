use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FetchMediaResponse {
    #[serde(rename = "num_results")]
    pub count: usize,
    pub items: Vec<Items>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items {
    pub pk: String,
    #[serde(rename = "product_type")]
    pub item_type: String,
    #[serde(rename = "original_width")]
    pub width: usize,
    #[serde(rename = "original_height")]
    pub height: usize,
    pub code: String,
    #[serde(rename = "taken_at")]
    pub timestamp: i64,
    #[serde(rename = "image_versions2")]
    pub media: Media,
    pub caption: Option<Caption>,
    pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Media {
    pub candidates: Vec<Candidate>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Candidate {
    pub width: usize,
    pub height: usize,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Caption {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub full_name: String,
    pub username: String,
}
