pub mod json;
pub mod datagen;

fn main() {
    let mut gen = datagen::news::Generator::new();
    println!("{:#?}", gen.create());
}
