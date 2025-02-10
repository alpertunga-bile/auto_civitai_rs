use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AutoCivitaiConfig {
    pub limit: u8,
    pub nsfw: String,
    pub sort: String,
    pub period: String,
    pub start_page: String,
}

pub fn from_config_to_url(config: &mut AutoCivitaiConfig) -> String {
    let mut base_url = Url::parse("https://civitai.com/api/v1/images").unwrap();

    base_url.set_query(Some(format!("limit={}", config.limit).as_str()));
    base_url.set_query(Some(format!("nsfw={}", config.nsfw).as_str()));
    base_url.set_query(Some(format!("sort={}", config.sort).as_str()));
    base_url.set_query(Some(format!("period={}", config.period).as_str()));
    base_url.set_query(Some(format!("cursor={}", config.start_page).as_str()));

    String::from(base_url)
}
