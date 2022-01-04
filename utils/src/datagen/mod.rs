mod news;
mod stonker;
mod company;
mod history;
mod stock;
mod command;

pub mod common;
pub use common::generator::Generator;
pub use common::index_vec::IndexVec;

use crate::json::NewsJSON;

use self::company::CompanyGenerator;
use self::news::NewsGenerator;
use self::stock::StockGenerator;
use self::stonker::StonkerGenerator;
use self::history::HistoryGenerator;
use self::command::CommandGenerator;

pub struct DataGenerator {
    company: CompanyGenerator,
    news: NewsGenerator,
    stocks: StockGenerator,
    stonkers: StonkerGenerator,
    history: HistoryGenerator,
    offers: CommandGenerator,
}

impl DataGenerator {
    pub fn new() -> anyhow::Result<DataGenerator> {
        Ok(DataGenerator {
            company: CompanyGenerator::new()?,
            news: NewsGenerator::new()?,
            stocks: StockGenerator::new(),
            stonkers: StonkerGenerator::new()?,
            history: HistoryGenerator::new(),
            offers: CommandGenerator::new(),
        })
    }
    pub fn create_news(&mut self) -> &NewsJSON {
        let company = self.company.create();
        self.news.create(company)
    }
}