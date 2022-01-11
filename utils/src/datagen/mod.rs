mod news;
mod stonker;
mod company;
mod history;
mod stock;
mod command;

pub mod common;
use std::convert::TryInto;
use std::process::Command;

pub use common::generator::Generator;
pub use common::index_vec::IndexVec;

use crate::json::{NewsJSON, CompanyJSON, StonkerJSON, HistoryJSON, CommandJSON, StockJSON};

use self::company::CompanyGenerator;
use self::news::NewsGenerator;
use self::stock::StockGenerator;
use self::stonker::StonkerGenerator;
use self::history::HistoryGenerator;
use self::command::CommandGenerator;

trait ToTSQL {
    fn to_header() -> &'static str;  // Stonker
    fn to_columns() -> Vec<&'static str>;  // [ id ] [ name ] [ password ]
    fn to_data(&self) -> Vec<String>; // frank1 "Frank" "knarf"
    fn convert(data: IndexVec<Self>) -> String where Self: Sized {
        format!("{{ {} }}\n{}\n{}\n", Self::to_header(),
            Self::to_columns().into_iter().map(|column| format!("[ {} ] ", column)).collect::<String>(),
            data.into_iter().map(|data|
                format!("{}\n", Self::to_data(&data).into_iter()
                    .map(|column| format!("{} ", column)).collect::<String>()),
            ).collect::<String>())
    }
}

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
}

struct Generators {
    companies: CompanyGenerator,
    news: NewsGenerator,
    stocks: StockGenerator,
    stonkers: StonkerGenerator,
    history: HistoryGenerator,
    commands: CommandGenerator,
}

impl Generators {
    pub fn new() -> anyhow::Result<Generators> {
        Ok(Generators {
            companies: CompanyGenerator::new()?,
            news: NewsGenerator::new()?,
            stocks: StockGenerator::new(),
            stonkers: StonkerGenerator::new()?,
            history: HistoryGenerator::new(),
            commands: CommandGenerator::new(),
        })
    }
}

pub struct DataGenerator {
    data: Data,
    generators: Generators
}

impl DataGenerator {
    pub fn new() -> anyhow::Result<DataGenerator> {
        Ok(DataGenerator {
            data: Data::new(),
            generators: Generators::new()?,
        })
    }
    fn company(&mut self) {
        self.generators.companies.create(&mut self.data)
    }
    fn news(&mut self) {
        self.generators.news.create(&mut self.data)
    }
    fn stonker(&mut self) {
        self.generators.stonkers.create(&mut self.data)
    }
    fn print(self) -> String {
        format!("{}{}{}",
            ToTSQL::convert(self.data.companies),
            ToTSQL::convert(self.data.stonkers),
            ToTSQL::convert(self.data.news)
        )
    }
    pub fn create(mut self) -> String {
        for _ in 0..5 {
            self.company()
        }
        for i in 0..10 {
            self.stonker();
        }
        for i in 0..3 {
            self.news();
        }
        self.print()
    }
}