use std::fs;

pub mod json;
pub mod datagen;

// this function is for experiments within utils module
#[allow(dead_code)]
fn main() -> Result<(), anyhow::Error> {
    let generator = datagen::DataGenerator::new()?;
    fs::write("./utils/data.tsql", generator.create())
        .expect("Unable to write file");
    Ok(())
}
