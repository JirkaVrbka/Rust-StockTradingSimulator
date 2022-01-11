mod news;
mod stonker;
mod company;
mod history;
mod stock;
mod command;

pub mod common;

pub use common::generator::Generator;
pub use common::index_vec::IndexVec;
pub use common::tsql::{ToTSQL, ToTSQLValue, TSQLValue};

use crate::json::{NewsJSON, CompanyJSON, StonkerJSON, HistoryJSON, CommandJSON, StockJSON};

use self::company::CompanyGenerator;
use self::news::NewsGenerator;
use self::stock::StockGenerator;
use self::stonker::StonkerGenerator;
use self::history::HistoryGenerator;
use self::command::CommandGenerator;

pub struct Data {
    companies: IndexVec<CompanyJSON>,
    news: IndexVec<NewsJSON>,
    stocks: IndexVec<StockJSON>,
    stonkers: IndexVec<StonkerJSON>,
    history: IndexVec<HistoryJSON>,
    commands: IndexVec<CommandJSON>
}

impl Data {
    fn new() -> Data {
        Data {
            companies: IndexVec::new(),
            news: IndexVec::new(),
            stocks: IndexVec::new(),
            stonkers: IndexVec::new(),
            history: IndexVec::new(),
            commands: IndexVec::new(),
        }
    }
    fn next(&self) -> i32 {
        (self.companies.len() + self.news.len() + self.stocks.len()
         + self.stonkers.len() + self.history.len() + self.commands.len()) as i32
    }
    fn to_string(self) -> String {
        format!("{}{}{}{}{}{}",
            ToTSQL::convert(self.stonkers),
            ToTSQL::convert(self.companies),
            ToTSQL::convert(self.stocks),
            ToTSQL::convert(self.news),
            ToTSQL::convert(self.history),
            ToTSQL::convert(self.commands),
        )
    }
}

trait JsonGenerator {
    fn new() -> anyhow::Result<Self> where Self: Sized;
    fn create(&mut self, generator: &mut Generator, data: &mut Data);
    fn n_times(&mut self, generator: &mut Generator, data: &mut Data, times: usize) {
        for _ in 0..times {
            self.create(generator, data);
        }
    }
}

struct Generators {
    companies: CompanyGenerator,
    news: NewsGenerator,
    stocks: StockGenerator,
    stonkers: StonkerGenerator,
    history: HistoryGenerator,
    commands: CommandGenerator,
}

pub struct MinCounts {
    pub companies: usize,
    pub news: usize,
    pub stocks: usize,
    pub stonkers: usize,
    pub history: usize,
    pub commands: usize,
}

impl Generators {
    pub fn new() -> anyhow::Result<Generators> {
        Ok(Generators {
            companies: CompanyGenerator::new()?,
            news: NewsGenerator::new()?,
            stocks: StockGenerator::new()?,
            stonkers: StonkerGenerator::new()?,
            history: HistoryGenerator::new()?,
            commands: CommandGenerator::new()?,
        })
    }
}

pub struct DataGenerator {
    data: Data,
    random: Generator,
    generators: Generators,
    min_counts: MinCounts
}

impl DataGenerator {
    pub fn new(min_counts: MinCounts) -> anyhow::Result<DataGenerator> {
        Ok(DataGenerator {
            data: Data::new(),
            random: Generator::new(),
            generators: Generators::new()?,
            min_counts
        })
    }
    pub fn create(mut self) -> String {
        self.generators.companies.n_times(&mut self.random, &mut self.data, self.min_counts.companies);
        self.generators.stocks.n_times(&mut self.random, &mut self.data, self.min_counts.stocks);
        self.generators.stonkers.n_times(&mut self.random, &mut self.data, 2 * self.min_counts.stonkers);
        self.generators.news.n_times(&mut self.random, &mut self.data, self.min_counts.news);
        self.generators.history.n_times(&mut self.random, &mut self.data, 10 * self.min_counts.history);
        self.data.to_string()
    }
}