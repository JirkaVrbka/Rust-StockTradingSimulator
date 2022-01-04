pub mod json;
pub mod datagen;

// this function is for experiments within utils module
#[allow(dead_code)]
fn main() -> Result<(), anyhow::Error> {
    let mut generator = datagen::DataGenerator::new()?;
    let news = generator.news();
    println!("{:#?}", news);
    Ok(())
}
