#[warn(unused_imports)]
mod dataset;
mod utilities;

use utilities::{
    config::{get_config, print_config, AutoCivitaiConfig},
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

    print_config(&config);

    let created_df = enhance_dataset(&config).await;

    if created_df.is_empty() {
        print!("{}", "\n".repeat(4));
        println!("Can not get new data with specified options, please update them and try again");
        return;
    }

    let mut df = get_dataframe(&config.output);
    df = postprocess_dataframe(df, created_df);

    save_dataframe(&config.output, &mut df);

    print!("{}", "\n".repeat(4));

    println!("Dataset is saved | Total Rows : {}", df.shape().0);
    println!("{}", df.head(Some(5)));
}
