use polars::{
    df,
    frame::UniqueKeepStrategy,
    io::SerReader,
    prelude::{DataFrame, ParquetReader, ParquetWriter},
};

pub fn get_dataframe(filepath: &str) -> DataFrame {
    if std::fs::exists(filepath).unwrap() {
        let mut file = std::fs::File::open(filepath).unwrap();
        let df = ParquetReader::new(&mut file).finish().unwrap();

        return df;
    }

    let df = df! [
      "prompt" => [None::<String>],
      "image_url" => [None::<String>],
      "base_model" => [None::<String>],
      "nsfw_level" => [None::<String>],
      "gen_type" => [None::<String>],
    ]
    .unwrap();

    return df;
}

pub fn postprocess_dataframe(real_df: DataFrame, created_df: DataFrame) -> DataFrame {
    let mut df = real_df.clone();
    let image_url_col = [String::from("image_url")];

    df = df.vstack(&created_df).unwrap();
    df = df.drop_nulls::<String>(None).unwrap();
    df = df
        .unique_stable(Some(&image_url_col), UniqueKeepStrategy::First, None)
        .unwrap();
    df.align_chunks_par();

    df
}

pub fn save_dataframe(filepath: &str, df: &mut DataFrame) {
    let mut file = std::fs::File::create(filepath).unwrap();

    ParquetWriter::new(&mut file).finish(df).unwrap();
}
