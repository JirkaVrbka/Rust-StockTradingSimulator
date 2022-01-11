use rand::Rng;
use serde::Deserialize;

use crate::json::{CompanyJSON, StonkerJSON};

use super::{Generator, IndexVec, Data, ToTSQL};

impl ToTSQL for CompanyJSON {
    fn to_header() -> &'static str {
        "Company"
    }
    fn to_columns() -> Vec<&'static str> {
        vec!["id", "name", "perfomer_id"]
    }
    fn to_data(&self) -> Vec<String> {
        vec![self.id.to_string(), self.name.to_string(), self.performer.id.to_string()]
    }
}

#[derive(Debug, Deserialize)]
struct Stock {
    symbol: String,
    name: String,
    price: f64
}

pub struct CompanyGenerator {
    generator: Generator,
    stocks: IndexVec<Stock>,
}

impl CompanyGenerator {
    pub fn new() -> anyhow::Result<CompanyGenerator> {
        Ok(CompanyGenerator{
            generator: Generator::new(),
            stocks: IndexVec::read_csv("stocks.csv", b',')?,
        })
    }
    pub fn create(&mut self, data: &mut Data) {
        let stock = self.generator.choose(&mut self.stocks);
        let id = self.generator.next();
        let performer = StonkerJSON {
            id,
            name: stock.name.clone(),
            balance: self.generator.random.gen_range(10_000..100_000),
            blocked_balance: 0,
            invested_balance: 0,
            password: self.generator.random_passwd(5),
        };
        data.stonkers.push_back(performer.clone());
        data.companies.push_back(CompanyJSON {
            id,
            name: stock.symbol.clone(),
            performer
        });
    }
}
