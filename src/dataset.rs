use polars::{
    df,
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
      "prompt" => [""],
      "media_url" => [""],
    ]
    .unwrap();

    return df;
}

pub fn save_dataframe(filepath: &str, df: &mut DataFrame) {
    let mut file = std::fs::File::create(filepath).unwrap();

    ParquetWriter::new(&mut file).finish(df).unwrap();
}
