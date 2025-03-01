use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize)]
pub struct AutoCivitaiConfig {
    pub limit: u8,
    pub nsfw: String,
    pub sort: String,
    pub period: String,
    pub start_page: u16,
    pub wanted_prompts: Vec<String>,
    pub unwanted_prompts: Vec<String>,
}

pub fn get_urls_from_config(config: &AutoCivitaiConfig, total_urls: u16) -> Vec<String> {
    let base_url = "https://civitai.com/api/v1/images";

    let mut index: u16 = config.start_page;

    let mut urls: Vec<String> = vec![];
    urls.reserve(total_urls as usize);

    let config_limit: u16 = config.limit as u16;

    while index < total_urls {
        let params = [
            ("limit", config.limit.to_string()),
            ("nsfw", config.nsfw.to_string()),
            ("sort", config.sort.to_string()),
            ("period", config.period.to_string()),
            ("cursor", index.to_string()),
        ];

        let url = Url::parse_with_params(base_url, params).unwrap();

        urls.push(url.to_string());

        index += config_limit;
    }

    urls
}

pub fn get_config(filepath: &str) -> Result<AutoCivitaiConfig, std::io::Error> {
    let config_file: String = String::from(filepath);

    let is_config_exists: bool = fs::exists(config_file.clone()).is_ok();

    if !is_config_exists {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("{} is not exists", filepath),
        ));
    }

    let config: AutoCivitaiConfig =
        serde_json::from_str(fs::read_to_string(config_file).unwrap().as_str()).unwrap();

    Ok(config)
}
