use polars::{
    df,
    io::SerReader,
    prelude::{DataFrame, ParquetReader},
};

fn get_dataframe(filepath: &str) -> DataFrame {
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
