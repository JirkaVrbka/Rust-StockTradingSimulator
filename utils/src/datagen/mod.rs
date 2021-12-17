pub mod news;

use std::fs::File;
use std::io::BufReader;
use anyhow::Error;
use csv::ReaderBuilder;
use serde::Deserialize;

pub fn read_csv<T: for<'de> Deserialize<'de>>(fname: &str, del: u8) -> Result<Vec<T>, Error> {
    let file = File::open(format!("./utils/datasets/{}", fname))?;
    let reader = BufReader::new(file);
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(del)
        .from_reader(reader);
    let mut vec = Vec::new();
    for result in csv_reader.deserialize() {
        let record: T = result?;
        vec.push(record)
    }
    Ok(vec)
}