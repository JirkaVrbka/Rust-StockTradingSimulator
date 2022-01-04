mod news;
mod stonkers;
mod company;
mod trades;
mod stocks;

pub mod common;
pub use common::generator::Generator;
pub use common::index_vec::IndexVec;

use self::company::CompanyGenerator;
use self::news::NewsGenerator;
use self::stocks::StockGenerator;
use self::stonkers::StonkerGenerator;
use self::trades::TradesGenerator;


pub struct DataGenerator {
    company: CompanyGenerator,
    news: NewsGenerator,
    stocks: StockGenerator,
    stonkers: StonkerGenerator,
    trades: TradesGenerator,
}

impl DataGenerator {
    fn new() -> anyhow::Result<DataGenerator> {
        Ok(DataGenerator {
            company: CompanyGenerator::new()?,
            news: NewsGenerator::new()?,
            stocks: StockGenerator::new(),
            stonkers: StonkerGenerator::new()?,
            trades: TradesGenerator::new()
        })
    }
}