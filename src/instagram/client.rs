use std::error::Error;

use instagram_media_shortcode::shortcode_to_ig_id;
use reqwest::{
    Client, ClientBuilder,
    header::{COOKIE, HeaderMap, HeaderValue, USER_AGENT},
};

use crate::instagram::{APP_ID, BASE_URL, FetchMediaResponse};

pub struct InstagramClient {
    client: Client,
}

impl InstagramClient {
    pub fn new(session_id: &str) -> Result<InstagramClient, Box<dyn Error>> {
        let mut headers = get_default_headers();
        headers.insert(COOKIE, HeaderValue::from_str(&session_id)?);
        let client = ClientBuilder::new().default_headers(headers).build()?;

        Ok(InstagramClient { client })
    }

    // todo: add reels and carousel support
    pub async fn fetch_media(&self, shortcode: &str) -> Result<FetchMediaResponse, Box<dyn Error>> {
        let media_id = shortcode_to_ig_id(shortcode)?;
        let url = format!("{}/media/{}/info/", BASE_URL, media_id);
        let response = self.client.get(&url).send().await?;
        let data = response.json::<FetchMediaResponse>().await?;

        Ok(data)
    }
}

fn get_default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Instagram 10.8.0 Android (18/4.3; 320dpi; 720x1280; Xiaomi; HM 1SW; armani; qcom; en_US)"));
    headers.insert(APP_ID, HeaderValue::from_static("936619743392459"));

    headers
}
