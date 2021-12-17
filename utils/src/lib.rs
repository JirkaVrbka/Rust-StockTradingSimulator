pub mod json;
pub mod datagen;

fn main() -> Result<(), anyhow::Error> {
    /*let mut gen = datagen::news::Generator::new()?;
    println!("{:#?}", gen.create());*/
    let mut gen = datagen::stonkers::StonkerGenerator::new()?;
    println!("{:#?}", gen.create());
    Ok(())
}
