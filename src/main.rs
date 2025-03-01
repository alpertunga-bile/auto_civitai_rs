#[warn(unused_imports)]
mod dataset;
mod utilities;

use utilities::{
    config::{get_config, AutoCivitaiConfig},
    enhance_dataset,
};

use dataset::{get_dataframe, postprocess_dataframe, save_dataframe};

#[tokio::main]
async fn main() {
    let config_result = get_config("config.json");
    let dataset_filepath = "dataset.parquet";

    if !config_result.is_ok() {
        println!("Error with config file");
        return;
    }

    let config: AutoCivitaiConfig = config_result.ok().unwrap();

    let mut df = get_dataframe(dataset_filepath);
    let created_df = enhance_dataset(&config).await;

    df = postprocess_dataframe(df, created_df);

    println!("\n\n\n{}", df);

    // save_dataframe(dataset_filepath, &mut df);
}
