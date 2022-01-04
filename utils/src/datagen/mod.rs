mod news;
mod stonker;
mod company;
mod history;
mod stock;
mod command;

pub mod common;
pub use common::generator::Generator;
pub use common::index_vec::IndexVec;

use crate::json::{NewsJSON, CompanyJSON, StonkerJSON};

use self::company::CompanyGenerator;
use self::news::NewsGenerator;
use self::stock::StockGenerator;
use self::stonker::StonkerGenerator;
use self::history::HistoryGenerator;
use self::command::CommandGenerator;

pub struct DataGenerator {
    companies: CompanyGenerator,
    news: NewsGenerator,
    stocks: StockGenerator,
    stonkers: StonkerGenerator,
    history: HistoryGenerator,
    offers: CommandGenerator,
}

impl DataGenerator {
    pub fn new() -> anyhow::Result<DataGenerator> {
        Ok(DataGenerator {
            companies: CompanyGenerator::new()?,
            news: NewsGenerator::new()?,
            stocks: StockGenerator::new(),
            stonkers: StonkerGenerator::new()?,
            history: HistoryGenerator::new(),
            offers: CommandGenerator::new(),
        })
    }
    pub fn company(&mut self) -> &CompanyJSON {
        self.companies.create()
    }
    pub fn news(&mut self) -> &NewsJSON {
        self.news.create(self.companies.create())
    }
    pub fn stonker(&mut self) -> &StonkerJSON {
        self.stonkers.create()
    }
}