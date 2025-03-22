use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Error, ErrorKind};

#[derive(Serialize, Deserialize)]
pub struct AutoCivitaiConfig {
    pub output: String,
    pub limit: u8,
    pub nsfw: String,
    pub sort: String,
    pub period: String,
    pub start_page: u16,
    pub total_pages: u16,
    pub wanted_prompts: Vec<String>,
    pub unwanted_prompts: Vec<String>,
}

pub fn get_urls_from_config(config: &AutoCivitaiConfig) -> Vec<String> {
    let base_url = "https://civitai.com/api/v1/images";

    let mut index = config.start_page;
    let config_limit = config.limit as u16;
    let index_limit = config.total_pages * config_limit;

    let mut urls: Vec<String> = Vec::with_capacity(config.total_pages as usize);

    while index < index_limit && index < 50000 {
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

    let mut config: AutoCivitaiConfig =
        serde_json::from_str(fs::read_to_string(config_file).unwrap().as_str()).unwrap();

    if !config.output.ends_with(".parquet") {
        config.output.push_str(".parquet");
    }

    Ok(config)
}

pub fn print_config(config: &AutoCivitaiConfig) {
    let mut begin = format!("{:-^100}", " Auto Civitai Config Values ");
    begin.replace_range(0..1, "+");
    begin.pop();
    begin.push('+');

    println!("{}", begin);

    println!(
        r#"
+ output           : {}
+ limit            : {}
+ nsfw             : {}
+ sort             : {}
+ period           : {}
+ start_page       : {}
+ total_pages      : {}
+ wanted_prompts   : {:?}
+ unwanted_prompts : {:?}
"#,
        config.output,
        config.limit,
        config.nsfw,
        config.sort,
        config.period,
        config.start_page,
        config.total_pages,
        config.wanted_prompts,
        config.unwanted_prompts
    );

    let mut end = String::from("+");
    end.push_str("-".repeat(98).as_str());
    end.push_str("+");

    println!("{}", end);
}
