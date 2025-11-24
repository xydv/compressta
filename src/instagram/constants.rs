use reqwest::header::HeaderName;

pub const BASE_URL: &str = "https://i.instagram.com/api/v1";
pub const APP_ID: HeaderName = HeaderName::from_static("x-ig-app-id");
