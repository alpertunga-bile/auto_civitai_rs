pub mod config;
mod prompt_utils;

use config::{get_urls_from_config, AutoCivitaiConfig};
use kdam::tqdm;
use prompt_utils::{get_page_image_data, ImageData, ImageDataVals};
use rand::prelude::*;
use reqwest::Client;
use serde_json::Value;
use tokio::time::Duration;

use polars::prelude::{Column, DataFrame};

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

            let _ = tokio::time::sleep(Duration::from_micros(
                rand::rng().random_range(2000000..5000000),
            ));

            let response = client.get(url).send().await?;
            response.json::<Value>().await
        };

        handles.push(tokio::spawn(future));
    }

    let mut results = Vec::with_capacity(handles.len());

    for handle in tqdm!(handles.into_iter(), desc = "Processing URLs") {
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

    for handle in tqdm!(handles.into_iter(), desc = "Processing", position = 0) {
        let mut image_data = handle.await.unwrap();

        if image_data.is_empty() {
            continue;
        }

        mul_image_data.append(&mut image_data);
    }

    mul_image_data
}

fn fill_dataframe(
    dataframe_data: &Vec<Vec<String>>,
    columns: &mut Vec<Column>,
    data_col: ImageDataVals,
) {
    let col_index = data_col as usize;

    match data_col {
        ImageDataVals::PROMPT => columns.push(Column::new(
            "prompt".into(),
            dataframe_data[col_index].clone(),
        )),
        ImageDataVals::IMAGE_URL => columns.push(Column::new(
            "image_url".into(),
            dataframe_data[col_index].clone(),
        )),
        ImageDataVals::BASE_MODEL => columns.push(Column::new(
            "base_model".into(),
            dataframe_data[col_index].clone(),
        )),
        ImageDataVals::NSFW_LEVEL => columns.push(Column::new(
            "nsfw_level".into(),
            dataframe_data[col_index].clone(),
        )),
        ImageDataVals::GEN_TYPE => columns.push(Column::new(
            "gen_type".into(),
            dataframe_data[col_index].clone(),
        )),
        ImageDataVals::TOTAL => {
            panic!("do not use ImageDataVals::TOTAL")
        }
    }
}

pub async fn enhance_dataset(config: &AutoCivitaiConfig) -> DataFrame {
    let urls = get_urls_from_config(&config, 1);

    let image_data = get_image_data(urls, &config.wanted_prompts, &config.unwanted_prompts).await;

    if image_data.len() == 0 {
        return DataFrame::default();
    }

    let total_image_vals = ImageDataVals::TOTAL as usize;

    let mut dataframe_data: Vec<Vec<String>> = Vec::with_capacity(total_image_vals);

    for _ in 0..total_image_vals {
        dataframe_data.push(Vec::new());
    }

    for data in tqdm!(
        image_data.into_iter(),
        desc = "Creating dataframe",
        position = 0
    ) {
        dataframe_data[ImageDataVals::PROMPT as usize].push(data.prompt);
        dataframe_data[ImageDataVals::IMAGE_URL as usize].push(data.image_url);
        dataframe_data[ImageDataVals::BASE_MODEL as usize].push(data.base_model);
        dataframe_data[ImageDataVals::NSFW_LEVEL as usize].push(data.nsfw_level);
        dataframe_data[ImageDataVals::GEN_TYPE as usize].push(data.gen_type);
    }

    let mut columns: Vec<Column> = Vec::with_capacity(total_image_vals);

    fill_dataframe(&dataframe_data, &mut columns, ImageDataVals::PROMPT);
    fill_dataframe(&dataframe_data, &mut columns, ImageDataVals::IMAGE_URL);
    fill_dataframe(&dataframe_data, &mut columns, ImageDataVals::BASE_MODEL);
    fill_dataframe(&dataframe_data, &mut columns, ImageDataVals::NSFW_LEVEL);
    fill_dataframe(&dataframe_data, &mut columns, ImageDataVals::GEN_TYPE);

    let df = DataFrame::new(columns);

    if df.is_err() {
        return DataFrame::default();
    }

    df.ok().unwrap()
}
