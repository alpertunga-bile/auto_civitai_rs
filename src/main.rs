mod config;
mod dataset;

use config::{get_config, get_urls_from_config, AutoCivitaiConfig};
use dataset::{get_dataframe, save_dataframe};
use futures::future;
use reqwest::get;
use tokio::time::Duration;

#[tokio::main]
async fn main() {
    let config_result = get_config("config.json");
    let dataset_filepath = "dataset.parquet";

    if !config_result.is_ok() {
        return;
    }

    let config_file: AutoCivitaiConfig = config_result.ok().unwrap();

    let urls = get_urls_from_config(&config_file);

    let mut df = get_dataframe(dataset_filepath);

    let bodies = future::join_all(urls.into_iter().map(|url| async move {
        tokio::time::sleep(Duration::from_secs(1)).await;
        get(url).await.unwrap().json::<serde_json::Value>().await
    }))
    .await;

    for b in bodies {
        match b {
            Ok(b) => println!(
                "Have {} items",
                b.get("items").unwrap().as_array().unwrap().len()
            ),
            Err(e) => eprintln!("Got an error: {}", e),
        }
    }

    save_dataframe(dataset_filepath, &mut df);
}
