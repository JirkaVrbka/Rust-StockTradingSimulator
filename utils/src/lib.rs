pub mod json;
pub mod datagen;

// this function is for experiments within utils module
#[allow(dead_code)]
fn main() -> Result<(), anyhow::Error> {
    let generator = datagen::DataGenerator::new()?;
    println!("{}", generator.create());
    Ok(())
}
