use rand::Rng;
use serde::Deserialize;

use crate::json::{CompanyJSON, StonkerJSON};

use super::{Generator, IndexVec};

#[derive(Debug, Deserialize)]
struct Stock {
    symbol: String,
    name: String,
    price: f64
}

pub struct CompanyGenerator {
    generator: Generator,
    stocks: IndexVec<Stock>,
    spawned: IndexVec<CompanyJSON>
}

impl CompanyGenerator {
    pub fn new() -> anyhow::Result<CompanyGenerator> {
        Ok(CompanyGenerator{
            generator: Generator::new(),
            stocks: IndexVec::read_csv("stocks.csv", b',')?,
            spawned: IndexVec::new()
        })
    }
    pub fn create(&mut self) -> &CompanyJSON {
        let stock = self.generator.choose(&mut self.stocks);
        let id = self.generator.next();
        self.spawned.push_back(CompanyJSON {
            id,
            name: stock.symbol.clone(),
            performer: StonkerJSON {
                id,
                name: stock.name.clone(),
                balance: self.generator.random.gen_range(10_000..100_000),
                blocked_balance: 0,
                invested_balance: 0,
            }
        })
    }
}
