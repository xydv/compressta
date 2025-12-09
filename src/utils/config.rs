use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompresstaConfig {
    pub instagram_cookie: String,
    pub merkle_tree: String,
    pub pinata_api_key: String,
    pub pinata_api_secret: String,
}
