pub mod json;
pub mod datagen;

// this function is for experiments within utils module
#[allow(dead_code)]
fn main() -> Result<(), anyhow::Error> {
    let mut company_gen = datagen::company::CompanyGenerator::new()?;
    let mut news_gen = datagen::news::NewsGenerator::new()?;
    let company = company_gen.create();
    let news = news_gen.create(company);
    println!("{:#?}", news);
    Ok(())
}
