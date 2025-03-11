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

    if config_result.is_err() {
        println!("Error with config file");
        return;
    }

    let config: AutoCivitaiConfig = config_result.ok().unwrap();

    let mut df = get_dataframe(&config.output);
    let created_df = enhance_dataset(&config).await;

    df = postprocess_dataframe(df, created_df);

    save_dataframe(&config.output, &mut df);

    print!("{}", "\n".repeat(4));

    println!("Dataset is saved | Total Rows : {}", df.shape().0);
    println!("{}", df.head(Some(5)));
}
