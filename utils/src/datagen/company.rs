use serde::Deserialize;

use crate::json::{CompanyJSON, StonkerJSON};

use super::{Generator, read_csv, IndexVec, push_back};

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
            stocks: read_csv::<Stock>("stocks.csv", b',')?,
            spawned: IndexVec::new()
        })
    }
    pub fn create(&mut self) -> &CompanyJSON {
        let stock = self.generator.choose(&mut self.stocks);
        let id = self.generator.next();
        push_back(&mut self.spawned, CompanyJSON {
            id,
            name: stock.symbol.clone(),
            performer: StonkerJSON {
                id,
                name: stock.name.clone(),
                balance: 0,
                blocked_balance: 0,
                invested_balance: 0,
            }
        })
    }
}