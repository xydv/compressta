use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct FetchMediaResponse {
    #[serde(rename = "num_results")]
    count: usize,
    items: Vec<Items>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Items {
    #[serde(rename = "product_type")]
    item_type: String,
    #[serde(rename = "original_width")]
    width: usize,
    #[serde(rename = "original_height")]
    height: usize,
    code: String,
    #[serde(rename = "taken_at")]
    timestamp: i64,
    #[serde(rename = "image_versions2")]
    media: Media,
}

#[derive(Serialize, Deserialize, Debug)]
struct Media {
    candidates: Vec<Candidate>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Candidate {
    width: usize,
    height: usize,
    url: String,
}
