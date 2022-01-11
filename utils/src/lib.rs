use std::fs;

use datagen::MinCounts;

pub mod json;
pub mod datagen;

// this function is for experiments within utils module
#[allow(dead_code)]
fn main() -> Result<(), anyhow::Error> {
    let generator = datagen::DataGenerator::new(MinCounts{
        companies: 1,
        news: 1,
        stocks: 1,
        stonkers: 1,
        history: 1,
        commands: 1,
    })?;
    let data_file = "./utils/data.tsql";
    fs::write(data_file, generator.create())
        .expect("Unable to write file");
    println!("Successfully written {}", data_file);
    Ok(())
}
