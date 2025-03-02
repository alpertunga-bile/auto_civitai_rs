pub mod config;
mod prompt_utils;

use config::{get_urls_from_config, AutoCivitaiConfig};
use prompt_utils::{
    get_page_image_data,
    image_data::{ImageData, ImageDataValues, ImageDataVectors},
};
use rand::prelude::*;
use reqwest::Client;
use serde_json::Value;
use tokio::time::Duration;
use tqdm::tqdm;

use polars::prelude::DataFrame;

async fn get_json_bodies(urls: Vec<String>) -> Vec<Result<Value, reqwest::Error>> {
    let client = Client::new();
    let url_size = urls.len();

    let mut handles = Vec::with_capacity(url_size);

    for url in urls.into_iter() {
        let client = client.clone();

        let future = async move {
            /*
             * wait between 2 seconds to 5 seconds
             * used microseconds to make it more random
             */

            let _ =
                tokio::time::sleep(Duration::from_micros(rand::rng().random_range(0..10000000)));

            let response = client.get(url).send().await?;
            response.json::<Value>().await
        };

        handles.push(tokio::spawn(future));
    }

    let mut results = Vec::with_capacity(handles.len());

    for handle in tqdm(handles.into_iter()).desc(Some("Processing URLs")) {
        results.push(handle.await.unwrap());
    }

    results
}

async fn get_image_data(
    urls: Vec<String>,
    wanted_prompts: &Vec<String>,
    unwanted_prompts: &Vec<String>,
) -> Vec<ImageData> {
    let url_size = urls.len();

    let json_bodies = get_json_bodies(urls).await;

    let mut handles = Vec::with_capacity(url_size);

    for body in json_bodies.into_iter() {
        let wanted = wanted_prompts.clone();
        let unwanted = unwanted_prompts.clone();

        match body {
            Ok(body) => {
                let future = async move {
                    let body_items = body.get("items");

                    if body_items.is_none() {
                        return Vec::new();
                    }

                    let items = body_items.unwrap().as_array().unwrap();

                    get_page_image_data(items, &wanted, &unwanted)
                };

                handles.push(tokio::spawn(future));
            }
            Err(e) => eprintln!("Got some error: {}", e),
        }
    }

    let mut mul_image_data: Vec<ImageData> = Vec::with_capacity(handles.len());

    for handle in tqdm(handles.into_iter()).desc(Some("Processing")) {
        let mut image_data = handle.await.unwrap();

        if image_data.is_empty() {
            continue;
        }

        mul_image_data.append(&mut image_data);
    }

    mul_image_data
}

pub async fn enhance_dataset(config: &AutoCivitaiConfig) -> DataFrame {
    let urls = get_urls_from_config(&config);

    let image_data = get_image_data(urls, &config.wanted_prompts, &config.unwanted_prompts).await;

    if image_data.len() == 0 {
        return DataFrame::default();
    }

    let total_image_vals = ImageDataValues::TotalValues as usize;
    let mut image_data_values = ImageDataVectors::new(total_image_vals);

    for data in tqdm(image_data.into_iter()).desc(Some("Creating Dataframe")) {
        image_data_values.append(&data);
    }

    image_data_values.create_dataframe(total_image_vals)
}
