mod civitai_image_struct;
mod config;

use civitai_image_struct::CivitaiImagePage;
use config::{from_config_to_url, AutoCivitaiConfig};
use reqwest::get;
use std::fs;

async fn fetch_data() -> Result<CivitaiImagePage, Box<dyn std::error::Error>> {
    let body: CivitaiImagePage = get("https://civitai.com/api/v1/images")
        .await?
        .json::<CivitaiImagePage>()
        .await?;

    Ok(body)
}

use std::io::{Error, ErrorKind};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config_file: String = String::from("config.json");

    let is_config_exists: bool = fs::exists(config_file.clone()).is_ok();

    if !is_config_exists {
        return Err(Error::new(
            ErrorKind::NotFound,
            "config.json file is not exists",
        ));
    }

    let config: AutoCivitaiConfig =
        serde_json::from_str(fs::read_to_string(config_file).unwrap().as_str()).unwrap();

    println!("{}", config.period);

    Ok(())
}
