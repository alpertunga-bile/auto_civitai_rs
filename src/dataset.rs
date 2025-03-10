use polars::{
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

    let df = DataFrame::default();

    return df;
}

pub fn postprocess_dataframe(real_df: DataFrame, created_df: DataFrame) -> DataFrame {
    let mut df = real_df.clone();
    let image_url_col = [String::from("url")];

    df = df.vstack(&created_df).unwrap();
    df = df
        .unique_stable(Some(&image_url_col), UniqueKeepStrategy::First, None)
        .unwrap();

    df
}

pub fn save_dataframe(filepath: &str, df: &mut DataFrame) {
    let mut file = std::fs::File::create(filepath).unwrap();

    ParquetWriter::new(&mut file).finish(df).unwrap();
}
